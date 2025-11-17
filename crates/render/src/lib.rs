pub mod meshing;

use asset::{Mesh, Vertex};
use glam::{vec2, vec3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

pub struct VoxelRenderer {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
}

impl VoxelRenderer {
    pub fn new(device: &Device, target_format: TextureFormat) -> Self {
        let shader_module = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vertex_layout = VertexBufferLayout {
            array_stride: 8 * 4,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: 3 * 4,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: 6 * 4,
                    shader_location: 2,
                    format: VertexFormat::Float32x2,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[vertex_layout],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let mut mesh = Mesh::new();
        mesh.add_vertex(Vertex {
            position: vec3(-1.0, 3.0, 0.0),
            normal: vec3(0.0, 0.0, 1.0),
            texcoord: vec2(0.0, 4.0),
        });
        mesh.add_vertex(Vertex {
            position: vec3(-1.0, -1.0, 0.0),
            normal: vec3(0.0, 0.0, 1.0),
            texcoord: vec2(0.0, 0.0),
        });
        mesh.add_vertex(Vertex {
            position: vec3(3.0, -1.0, 0.0),
            normal: vec3(0.0, 0.0, 1.0),
            texcoord: vec2(4.0, 0.0),
        });

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.vertex_data()),
            usage: BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
        }
    }

    pub fn render(&self, rp: &mut RenderPass) {
        rp.set_pipeline(&self.pipeline);
        rp.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rp.draw(0..3, 0..1);
    }
}
