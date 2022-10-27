use cgmath::Rotation3;
use wgpu::util::DeviceExt;

use crate::{
    instance::{self, Instance},
    model::{self, ModelVertex, Vertex},
    texture, MapTiles,
};

pub(crate) fn create_buffers(
    device: &wgpu::Device,
    vertexes: &[ModelVertex],
    indices: &[u16],
) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertexes),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = indices.len() as u32;
    (vertex_buffer, index_buffer, num_indices)
}

pub(crate) fn init_config(
    surface: &wgpu::Surface,
    adapter: wgpu::Adapter,
    size: winit::dpi::PhysicalSize<u32>,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    }
}

pub(crate) fn texture_bind_group_layout_init(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
    })
}

pub(crate) fn camera_bind_init(
    device: &wgpu::Device,
    camera_buffer: &wgpu::Buffer,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
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
    (camera_bind_group_layout, camera_bind_group)
}

pub(crate) fn create_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_bytes: &[u8],
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    let img_texture =
        texture::Texture::from_bytes(device, queue, texture_bytes, "texture").unwrap();
    let img_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&img_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&img_texture.sampler),
            },
        ],
        label: Some("wall_bind_group"),
    });
    img_bind_group
}

pub(crate) fn pipeline_init(
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
                //blend: Some(wgpu::BlendState::REPLACE),
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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

pub(crate) fn instance_init(
    device: &wgpu::Device,
    tiles: MapTiles,
    width: f32,
    height: f32,
    depth: f32,
) -> (Vec<Instance>, wgpu::Buffer) {
    let instances = (0..tiles.depth)
        .flat_map(|z| {
            (0..tiles.width)
                .map(move |x| (x, z))
                .flat_map(|(x, z)| (0..tiles.map[z * tiles.width + x]).map(move |y| (x, y, z)))
        })
        .map(|(x, y, z)| {
            (
                2.0 * width * x as f32,
                2.0 * height * y as f32,
                2.0 * depth * z as f32,
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

pub(crate) fn slope_instance_init(
    device: &wgpu::Device,
    tiles: MapTiles,
    width: f32,
    height: f32,
    depth: f32,
) -> (Vec<Instance>, wgpu::Buffer) {
    let instances = (0..tiles.depth)
        .flat_map(|z| {
            (0..tiles.width)
                .map(move |x| (x, z))
                .flat_map(|(x, z)| (0..tiles.map[z * tiles.width + x]).map(move |y| (x, y, z)))
        })
        .map(|(x, y, z)| {
            (
                2.0 * width * x as f32,
                (height - 1.0) + (2.0 * height * y as f32),
                2.0 * depth * z as f32,
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
