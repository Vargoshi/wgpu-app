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
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use cgmath::prelude::*;

use crate::{
    instance::{Instance, NUM_INSTANCES_PER_ROW},
    model::{DrawModel, Vertex},
};

// const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [0.5, -0.5, 0.0],
//         color: [0.0, 0.0, 1.0],
//     }, // bottom right
//     Vertex {
//         position: [0.5, 0.5, 0.0],
//         color: [0.0, 1.0, 0.0],
//     }, // top right
//     Vertex {
//         position: [-0.5, 0.5, 0.0],
//         color: [1.0, 0.0, 0.0],
//     }, // top left
//     Vertex {
//         position: [-0.5, -0.5, 0.0],
//         color: [0.5, 0.5, 0.5],
//     }, // bottom left
// ];

// const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

// const VERTICES: &[vertex::Vertex] = &[
//     // Changed
//     vertex::Vertex {
//         position: [-0.0868241, 0.49240386, 0.0],
//         tex_coords: [0.4131759, 0.00759614],
//     }, // A
//     vertex::Vertex {
//         position: [-0.49513406, 0.06958647, 0.0],
//         tex_coords: [0.0048659444, 0.43041354],
//     }, // B
//     vertex::Vertex {
//         position: [-0.21918549, -0.44939706, 0.0],
//         tex_coords: [0.28081453, 0.949397],
//     }, // C
//     vertex::Vertex {
//         position: [0.35966998, -0.3473291, 0.0],
//         tex_coords: [0.85967, 0.84732914],
//     }, // D
//     vertex::Vertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         tex_coords: [0.9414737, 0.2652641],
//     }, // E
// ];

// const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

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
    let window = WindowBuilder::new().build(&event_loop).unwrap();

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

    //let diffuse_bytes = include_bytes!("happy-tree.png");
    //let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    //let diffuse_rgba = diffuse_image.to_rgba8();

    //use image::GenericImageView;
    //let dimensions = diffuse_image.dimensions();

    // let texture_size = wgpu::Extent3d {
    //     width: dimensions.0,
    //     height: dimensions.1,
    //     depth_or_array_layers: 1,
    // };

    // let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
    //     // All textures are stored as 3D, we represent our 2D texture
    //     // by setting depth to 1.
    //     size: texture_size,
    //     mip_level_count: 1, // We'll talk about this a little later
    //     sample_count: 1,
    //     dimension: wgpu::TextureDimension::D2,
    //     // Most images are stored using sRGB so we need to reflect that here.
    //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
    //     // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
    //     // COPY_DST means that we want to copy data to this texture
    //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    //     label: Some("diffuse_texture"),
    // });

    // let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
    // let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    //     address_mode_u: wgpu::AddressMode::ClampToEdge,
    //     address_mode_v: wgpu::AddressMode::ClampToEdge,
    //     address_mode_w: wgpu::AddressMode::ClampToEdge,
    //     mag_filter: wgpu::FilterMode::Linear,
    //     min_filter: wgpu::FilterMode::Nearest,
    //     mipmap_filter: wgpu::FilterMode::Nearest,
    //     ..Default::default()
    // });

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

    // let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //     layout: &texture_bind_group_layout,
    //     entries: &[
    //         wgpu::BindGroupEntry {
    //             binding: 0,
    //             resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
    //         },
    //         wgpu::BindGroupEntry {
    //             binding: 1,
    //             resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
    //         },
    //     ],
    //     label: Some("diffuse_bind_group"),
    // });

    let mut camera = camera::Camera {
        // position the camera one unit up and 2 units back
        // +z is out of the screen
        eye: (0.0, 1.0, 2.0).into(),
        // have it look at the origin
        target: (0.0, 0.0, 0.0).into(),
        // which way is "up"
        up: cgmath::Vector3::unit_y(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut camera_uniform = camera_uniform::CameraUniform::new();
    camera_uniform.update_view_proj(&camera);

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

    let mut camera_controller = camera_controller::CameraController::new(0.2);

    const SPACE_BETWEEN: f32 = 3.0;
    let instances = (0..NUM_INSTANCES_PER_ROW)
        .flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance { position, rotation }
            })
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

    let mut depth_texture =
        texture::Texture::create_depth_texture(&device, &config, "depth_texture");

    let obj_model =
        resources::load_model("cube.obj", &device, &queue, &texture_bind_group_layout).unwrap();

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

    // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Vertex Buffer"),
    //     contents: bytemuck::cast_slice(VERTICES),
    //     usage: wgpu::BufferUsages::VERTEX,
    // });

    // let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Index Buffer"),
    //     contents: bytemuck::cast_slice(INDICES),
    //     usage: wgpu::BufferUsages::INDEX,
    // });

    // let num_indices = INDICES.len() as u32;

    let mut mouse_pos = (0.0, 0.0);

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                camera_controller.process_events(&event);
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }

                    WindowEvent::Resized(size) => {
                        if size.width * size.height > 0 {
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

                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.draw_model_instanced(
                        &obj_model,
                        0..instances.len() as u32,
                        &camera_bind_group,
                    );
                }

                // submit will accept anything that implements IntoIter

                // queue.write_texture(
                //     // Tells wgpu where to copy the pixel data
                //     wgpu::ImageCopyTexture {
                //         texture: &diffuse_texture,
                //         mip_level: 0,
                //         origin: wgpu::Origin3d::ZERO,
                //         aspect: wgpu::TextureAspect::All,
                //     },
                //     // The actual pixel data
                //     &diffuse_rgba,
                //     // The layout of the texture
                //     wgpu::ImageDataLayout {
                //         offset: 0,
                //         bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                //         rows_per_image: std::num::NonZeroU32::new(dimensions.1),
                //     },
                //     texture_size,
                // );

                camera_controller.update_camera(&mut camera);
                camera_uniform.update_view_proj(&camera);
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
