use wgpu::{BindGroupLayout, VertexBufferLayout};

use crate::{
    bind::BindGroup,
    gpu::Gpu,
    vertex::{Vertex, Vertex2},
};

pub struct ShaderBuilder<'a> {
    label: Option<String>,
    source: wgpu::ShaderSource<'a>,
}
impl<'a> Default for ShaderBuilder<'a> {
    fn default() -> Self {
        Self {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("missing.wgsl").into()),
        }
    }
}
impl<'a> ShaderBuilder<'a> {
    pub fn build(&self, gpu: &Gpu) -> Shader {
        let wgpu_shader = gpu
            .get_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: self.label.as_deref(),
                source: self.source.clone(),
            });
        Shader {
            handle: wgpu_shader,
        }
    }
}

pub struct Shader {
    handle: wgpu::ShaderModule,
}

#[derive(Default)]
pub struct PipelineLayoutBuilder {
    label: Option<String>,
}
impl PipelineLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    pub fn build(&self, gpu: &Gpu, bind_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        let layout = gpu
            .get_device()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label.as_deref(),
                bind_group_layouts: bind_layouts,
                push_constant_ranges: &[],
            });
        PipelineLayout { handle: layout }
    }
}

pub struct PipelineLayout {
    handle: wgpu::PipelineLayout,
}

pub struct PipelineBuilder<'a> {
    label: Option<String>,
    shader: ShaderBuilder<'a>,
    layout: PipelineLayoutBuilder,
    line_mode: bool,
    vertex_layout: VertexBufferLayout<'static>,
}
impl<'a> Default for PipelineBuilder<'a> {
    fn default() -> Self {
        Self {
            label: Default::default(),
            shader: ShaderBuilder::default(),
            layout: PipelineLayoutBuilder::default(),
            line_mode: false,
            vertex_layout: Vertex2::get_layout(),
        }
    }
}
impl<'a> PipelineBuilder<'a> {
    pub fn build_2d_default(gpu: &Gpu, bind_group: &BindGroup) -> Pipeline {
        Self {
            label: Some("2d pipeline".into()),
            shader: ShaderBuilder {
                label: Some("2d shader".into()),
                source: wgpu::ShaderSource::Wgsl(include_str!("2d.wgsl").into()),
            },
            layout: PipelineLayoutBuilder {
                label: Some("2d layout".into()),
            },
            line_mode: false,
            vertex_layout: Vertex2::get_layout(),
        }
        .build(gpu, Some(bind_group))
    }

    pub fn build_2d_lines_default(gpu: &Gpu, bind_group: &BindGroup) -> Pipeline {
        Self {
            label: Some("2d pipeline".into()),
            shader: ShaderBuilder {
                label: Some("2d shader".into()),
                source: wgpu::ShaderSource::Wgsl(include_str!("2d.wgsl").into()),
            },
            layout: PipelineLayoutBuilder {
                label: Some("2d layout".into()),
            },
            line_mode: true,
            vertex_layout: Vertex2::get_layout(),
        }
        .build(gpu, Some(bind_group))
    }

    pub fn build(&self, gpu: &Gpu, bind_layout: Option<&BindGroup>) -> Pipeline {
        let device = gpu.get_device();
        let config = gpu.get_config();

        let bind_layouts = match bind_layout {
            Some(bind_layout) => vec![bind_layout.get_layout()],
            None => Vec::new(),
        };

        let layout = self.layout.build(gpu, &bind_layouts);
        let shader = self.shader.build(gpu);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label.as_deref(),
            layout: Some(&layout.handle),
            vertex: wgpu::VertexState {
                module: &shader.handle,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: std::slice::from_ref(&self.vertex_layout),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.handle,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: if !self.line_mode {
                    wgpu::PrimitiveTopology::TriangleList
                } else {
                    wgpu::PrimitiveTopology::LineStrip
                },
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
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
        Pipeline {
            wgpu_pipeline: pipeline,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    wgpu_pipeline: wgpu::RenderPipeline,
}
impl Pipeline {
    pub fn get_handle(&self) -> &wgpu::RenderPipeline {
        &self.wgpu_pipeline
    }
}
