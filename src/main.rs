#![deny(clippy::all)]

mod camera;
mod camera_controller;
mod camera_uniform;
mod instance;
mod model;
mod resources;
mod texture;
mod vertex;

use wgpu::util::DeviceExt;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use cgmath::prelude::*;

use crate::{
    instance::Instance,
    model::{DrawModel, Vertex},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompareFunction {
    Undefined = 0,
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}

fn main() {
    env_logger::init(); // Necessary for logging within WGPU
    let event_loop = EventLoop::new(); // Loop provided by winit for handling window events
    let window = WindowBuilder::new().build(&event_loop).unwrap(); // Create a window centered around the Loop

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None, // Trace path
    ))
    .unwrap();

    let size = window.inner_size();
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let mut camera = camera::Camera::new((5.0, 0.0, 6.0), cgmath::Deg(-90.0), cgmath::Deg(0.0));
        let mut projection = camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let mut camera_controller = camera_controller::CameraController::new(4.0,0.4);
    
        let mut camera_uniform = camera_uniform::CameraUniform::new();
        camera_uniform.update_view_proj(&camera,&projection);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

    let (instances, instance_buffer) = instance_init(&device);
    let (instances2, instance_buffer2) = instance_init2(&device);

    let mut depth_texture =
        texture::Texture::create_depth_texture(&device, &config, "depth_texture");

    let obj_model =
        resources::load_model("wallcube.obj", &device, &queue, &texture_bind_group_layout).unwrap();

    let obj_model2 =
        resources::load_model("floortile.obj", &device, &queue, &texture_bind_group_layout)
            .unwrap();

    let render_pipeline = pipeline_init(
        &device,
        texture_bind_group_layout,
        camera_bind_group_layout,
        &config,
    );

    let mut mouse_pos = (0.0, 0.0);

    let mut last_render_time = instant::Instant::now();

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => if camera_controller.mouse_pressed {
                camera_controller.process_mouse(delta.0, delta.1)
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() && !camera_controller.process_events(&event)=> {
                camera_controller.process_events(&event);
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }

                    WindowEvent::Resized(size) => {
                        if size.width * size.height > 0 {
                            projection.resize(size.width,size.height);
                            config.width = size.width;
                            config.height = size.height;
                            surface.configure(&device, &config);
                        }
                        depth_texture = texture::Texture::create_depth_texture(
                            &device,
                            &config,
                            "depth_texture",
                        );
                    }

                    WindowEvent::ScaleFactorChanged {
                        new_inner_size: size,
                        ..
                    } => {
                        if size.width * size.height > 0 {
                            config.width = size.width;
                            config.height = size.height;
                            surface.configure(&device, &config);
                        }
                    }

                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                            *control_flow = ControlFlow::Exit
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_pos = (position.x, position.y);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                
                let output = surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1, // Pick any color you want here
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_texture.view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        }),
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass.draw_model_instanced(
                        &obj_model,
                        0..instances.len() as u32,
                        &camera_bind_group,
                    );
                    render_pass.set_vertex_buffer(1, instance_buffer2.slice(..));
                    render_pass.draw_model_instanced(
                        &obj_model2,
                        0..instances2.len() as u32,
                        &camera_bind_group,
                    );
                }

                camera_controller.update_camera(&mut camera, dt);
                camera_uniform.update_view_proj(&camera, &projection);
                queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
            }

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }

            _ => (),
        }
    });
}

fn pipeline_init(
    device: &wgpu::Device,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    camera_bind_group_layout: wgpu::BindGroupLayout,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
        push_constant_ranges: &[],
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[model::ModelVertex::desc(), instance::InstanceRaw::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: texture::Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // 1.
            stencil: wgpu::StencilState::default(),     // 2.
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });
    render_pipeline
}

fn instance_init(device: &wgpu::Device) -> (Vec<Instance>, wgpu::Buffer) {
    #[rustfmt::skip]
    let height_map = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 
        1, 0, 0, 1, 0, 0, 0, 1, 
        1, 0, 0, 1, 0, 1, 0, 1, 
        1, 1, 1, 1, 0, 0, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 0, 0, 0, 1, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 1, 
        1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let map_block_width = 8;
    let map_block_depth = 8;

    const BLOCK_SIZE: f32 = 2.0;
    let instances = (0..map_block_depth)
        .flat_map(|z| {
            (0..map_block_width)
                .map(move |x| (x, z))
                .flat_map(|(x, z)| (0..height_map[z * map_block_width + x]).map(move |y| (x, y, z)))
        })
        .map(|(x, y, z)| {
            (
                BLOCK_SIZE * x as f32,
                BLOCK_SIZE * y as f32,
                BLOCK_SIZE * z as f32,
            )
        })
        .map(|(x, y, z)| Instance {
            position: cgmath::Vector3 { x, y, z },
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
        })
        .collect::<Vec<_>>();

    let instance_data = instances
        .iter()
        .map(instance::Instance::to_raw)
        .collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    (instances, instance_buffer)
}

fn instance_init2(device: &wgpu::Device) -> (Vec<Instance>, wgpu::Buffer) {
    #[rustfmt::skip]
    let height_map = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 1, 1, 0, 1, 1, 1, 0, 
        0, 1, 1, 0, 1, 0, 1, 0, 
        0, 0, 0, 0, 1, 1, 1, 0, 
        0, 1, 1, 1, 1, 1, 1, 0, 
        0, 1, 1, 1, 1, 0, 1, 0, 
        0, 1, 1, 1, 1, 1, 1, 0, 
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let map_block_width = 8;
    let map_block_depth = 8;

    const BLOCK_SIZE: f32 = 2.0;
    let instances = (0..map_block_depth)
        .flat_map(|z| {
            (0..map_block_width)
                .map(move |x| (x, z))
                .flat_map(|(x, z)| (0..height_map[z * map_block_width + x]).map(move |y| (x, y, z)))
        })
        .map(|(x, y, z)| {
            (
                BLOCK_SIZE * x as f32,
                BLOCK_SIZE * y as f32,
                BLOCK_SIZE * z as f32,
            )
        })
        .map(|(x, y, z)| Instance {
            position: cgmath::Vector3 { x, y, z },
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
        })
        .collect::<Vec<_>>();

    let instance_data = instances
        .iter()
        .map(instance::Instance::to_raw)
        .collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    (instances, instance_buffer)
}

