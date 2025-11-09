use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, vec2, vec3};
use pollster::FutureExt;
use wgpu::{AdapterInfo, CommandEncoderDescriptor, TextureViewDescriptor};
use wgpu::util::DeviceExt;
use wgpu::{
    Adapter, Buffer, BufferUsages, Color, Device, DeviceDescriptor, FragmentState, Instance,
    InstanceDescriptor, LoadOp, Operations, PipelineLayoutDescriptor, PowerPreference,
    PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderSource, StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
    surface: Surface<'static>,
    adapter: Adapter,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,

    render_pipeline: RenderPipeline,
    mesh_buffer: MeshBuffer,

    window: Window,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());

        // SAFETY: Window has the same lifetime as surface
        let surface = unsafe {
            instance
                .create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap())
                .unwrap()
        };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();

        let inner_size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, inner_size.width, inner_size.height)
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .block_on()
            .unwrap();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::layout()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[
                Vertex {
                    position: vec3(-0.5, -0.5, 0.0),
                    normal: vec3(0.0, 0.0, 1.0),
                    texcoord: vec2(0.0, 0.0),
                },
                Vertex {
                    position: vec3(0.5, -0.5, 0.0),
                    normal: vec3(0.0, 0.0, 1.0),
                    texcoord: vec2(1.0, 0.0),
                },
                Vertex {
                    position: vec3(0.0, 0.5, 0.0),
                    normal: vec3(0.0, 0.0, 1.0),
                    texcoord: vec2(0.5, 1.0),
                },
            ]),
            usage: BufferUsages::VERTEX,
        });

        let mesh_buffer = MeshBuffer {
            vertex_buffer,
            index_buffer: None,
            num_indices: 0,
            num_vertices: 3,
        };

        let mut renderer = Self {
            surface,
            adapter,
            surface_config,
            device,
            queue,

            render_pipeline,
            mesh_buffer,

            window,
        };

        renderer.resize(inner_size);

        renderer
    }

    pub fn adapter_info(&self) -> AdapterInfo {
        self.adapter.get_info()
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        let surface_texture = self.surface.get_current_texture().unwrap();
        let surface_texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });


            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_vertex_buffer(0, self.mesh_buffer.vertex_buffer.slice(..));
            if let Some(index_buffer) = &self.mesh_buffer.index_buffer {
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..self.mesh_buffer.num_indices, 0, 0..1);
            } else {
                render_pass.draw(0..self.mesh_buffer.num_vertices, 0..1);
            }
        }

        self.queue.submit([encoder.finish()]);

        surface_texture.present();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}

pub struct MeshBuffer {
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
    num_indices: u32,
    num_vertices: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 3] = [
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
    ];

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
