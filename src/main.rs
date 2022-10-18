#![deny(clippy::all)]

mod camera;
mod collision_detection;
mod cube;
mod floor;
mod camera_controller;
mod camera_uniform;
mod instance;
mod model;
mod texture;
mod systems;

use std::time::Instant;

use cube::Cube;
use floor::Floor;


use systems::*;
use wgpu::util::DeviceExt;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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

struct MapTiles {
    map: Vec<i32>,
    width: usize,
    depth: usize
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

    let mut config = init_config(&surface, adapter, size);

    surface.configure(&device, &config);

    let mut camera = camera::Camera::new((5.0, 1.0, 6.0), cgmath::Deg(-90.0), cgmath::Deg(0.0)); //init position of the camera
    let mut projection = camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
    let mut camera_controller = camera_controller::CameraController::new(4.0,0.4);
    
    let mut camera_uniform = camera_uniform::CameraUniform::new();
    camera_uniform.update_view_proj(&camera,&projection);
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
    let (camera_bind_group_layout, camera_bind_group) = camera_bind_init(&device, &camera_buffer);
        
    let texture_bind_group_layout = texture_bind_group_layout_init(&device);
    let mut depth_texture =
        texture::Texture::create_depth_texture(&device, &config, "depth_texture");


    let mut cube = Cube::new(2.0,1.0,2.0);
    let wall_bytes = include_bytes!("wall.png");
    let wall_bind_group = create_texture(&device, &queue, wall_bytes, &texture_bind_group_layout);
    let (wall_vertex_buffer, wall_index_buffer, wall_num_indices) = create_buffers(&device, &cube.vertexes, &cube.indices);
        
    let walls = MapTiles{
            map: vec![
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 0, 0, 2, 0, 0, 0, 2, 
                2, 0, 0, 2, 0, 1, 0, 2, 
                2, 2, 0, 2, 0, 0, 0, 2, 
                2, 0, 0, 0, 0, 0, 0, 2, 
                2, 0, 0, 0, 0, 1, 0, 2, 
                2, 0, 0, 0, 0, 0, 0, 2, 
                2, 2, 2, 2, 2, 2, 2, 2,
            ],
            width: 8,
            depth: 8
        };

    let (instances, instance_buffer) = instance_init(&device, walls, cube.width, cube.height, cube.depth);
        
    let mut floor = Floor::new(2.0,1.0, 2.0);
    let floor_bytes = include_bytes!("floor.png");
    let floor_bind_group = create_texture(&device, &queue, floor_bytes, &texture_bind_group_layout);
    let (floor_vertex_buffer, floor_index_buffer, floor_num_indices) = create_buffers(&device, &floor.vertexes, &floor.indices);

    let floor_tiles = MapTiles{
        map: vec![
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 1, 1, 0, 1, 1, 1, 0, 
            0, 1, 1, 0, 1, 0, 1, 0, 
            0, 0, 1, 0, 1, 1, 1, 0, 
            0, 1, 1, 1, 1, 1, 1, 0, 
            0, 1, 1, 1, 1, 0, 1, 0, 
            0, 1, 1, 1, 1, 1, 1, 0, 
            0, 0, 0, 0, 0, 0, 0, 0,
        ],
        width: 8,
        depth: 8
    };

    let (instances2, instance_buffer2) = instance_init(&device, floor_tiles, floor.width, floor.height, floor.depth);

    let render_pipeline = pipeline_init(
        &device,
        texture_bind_group_layout,
        camera_bind_group_layout,
        &config,
    );

    let time = Instant::now();
    let mut frame1 = 0;

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.run(move |event, _, control_flow| {

        match event {

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => if camera_controller.mouse_pressed {
                camera_controller.process_mouse(delta.0, delta.1);
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() => {
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
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let frame2 = time.elapsed().as_millis();
                let dt = frame2-frame1;
                frame1 = time.elapsed().as_millis();
                
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
                    render_pass.set_bind_group(1, &camera_bind_group, &[]);

                    render_pass.set_bind_group(0, &wall_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, wall_vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass.set_index_buffer(wall_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..wall_num_indices, 0, 0..instances.len() as _);


                    render_pass.set_bind_group(0, &floor_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, floor_vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_buffer2.slice(..));
                    render_pass.set_index_buffer(floor_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..floor_num_indices, 0, 0..instances2.len() as _);
                    
                }

                camera_controller.update_camera(&mut camera, dt, &mut cube, &mut floor, instances.as_slice(), instances2.as_slice());
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
