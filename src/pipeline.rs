use wgpu::{PipelineLayoutDescriptor, ShaderModuleDescriptor};

#[derive(Default)]
pub struct ShaderBuilder<'a> {
    label: Option<String>,
    source: Option<wgpu::ShaderSource<'a>>,
}
impl<'a> ShaderBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_source(&mut self, source: wgpu::ShaderSource<'a>) -> &mut Self {
        self.source = Some(source);
        self
    }

    pub fn build(&self, device: &wgpu::Device) -> Shader {
        let source = self.source.clone().expect("source is expected to be set");
        let wgpu_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: self.label.as_deref(),
            source,
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
pub struct LayoutBuilder {
    label: Option<String>,
}
impl LayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    pub fn build(&self, device: &wgpu::Device) -> Layout {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: self.label.as_deref(),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        Layout { handle: layout }
    }
}

pub struct Layout {
    handle: wgpu::PipelineLayout,
}

#[derive(Default)]
pub struct PipelineBuilder {
    label: Option<String>,
    shader: Option<Shader>,
    layout: Option<Layout>,
}
impl PipelineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_shader(&mut self, shader: Shader) -> &mut Self {
        self.shader = Some(shader);
        self
    }

    pub fn with_layout(&mut self, layout: Layout) -> &mut Self {
        self.layout = Some(layout);
        self
    }

    pub fn build(&self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Pipeline {
        let shader = self.shader.as_ref().expect("shader must be set");
        let layout = self.layout.as_ref().expect("layout must be set");

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label.as_deref(),
            layout: Some(&layout.handle),
            vertex: wgpu::VertexState {
                module: &shader.handle,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.handle,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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

pub struct Pipeline {
    wgpu_pipeline: wgpu::RenderPipeline,
}
impl Pipeline {
    pub fn get_handle(&self) -> &wgpu::RenderPipeline {
        &self.wgpu_pipeline
    }
}
