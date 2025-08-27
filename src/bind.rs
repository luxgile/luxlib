use crate::gpu::Gpu;

pub trait BindEntry {
    fn visibility(&self) -> wgpu::ShaderStages;
    fn resource(&self) -> wgpu::BindingResource<'_>;
    fn ty(&self) -> wgpu::BindingType;
}

pub struct BindLayout;

pub struct BindGroupBuilder {
    label: Option<String>,
    layout: BindLayout,
    entries: Vec<Box<dyn BindEntry>>,
}
impl Default for BindGroupBuilder {
    fn default() -> Self {
        Self {
            label: None,
            layout: BindLayout,
            entries: Vec::new(),
        }
    }
}
impl BindGroupBuilder {
    pub fn entry(&mut self, entry: impl BindEntry + 'static) -> &mut Self {
        self.entries.push(Box::new(entry));
        self
    }

    pub fn build(&self, gpu: &Gpu) -> BindGroup {
        let mut layout_entries = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: entry.visibility(),
                ty: entry.ty(),
                count: None,
            });
        }
        let layout = gpu
            .get_device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: self.label.as_deref(),
                entries: &layout_entries,
            });

        let mut entries = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            entries.push(wgpu::BindGroupEntry {
                binding: i as u32,
                resource: entry.resource(),
            });
        }

        let bind_group = gpu
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label.as_deref(),
                layout: &layout,
                entries: &entries,
            });

        BindGroup { handle: bind_group }
    }
}

#[derive(Debug, Clone)]
pub struct BindGroup {
    handle: wgpu::BindGroup,
}
impl BindGroup {
    pub fn get_handle(&self) -> &wgpu::BindGroup {
        &self.handle
    }
}
