use bitflags::bitflags;

use crate::gpu::Gpu;

pub trait BindEntry {
    fn resource(&self) -> wgpu::BindingResource<'_>;
    fn ty(&self) -> wgpu::BindingType;
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct ShaderStage : u16 {
        const VERTEX = 1 << 1;
        const FRAGMENT = 1 << 2;
    }
}
impl From<ShaderStage> for wgpu::ShaderStages {
    fn from(value: ShaderStage) -> Self {
        let mut stage = wgpu::ShaderStages::empty();
        if value.contains(ShaderStage::VERTEX) {
            stage = stage.union(wgpu::ShaderStages::VERTEX);
        }
        if value.contains(ShaderStage::FRAGMENT) {
            stage = stage.union(wgpu::ShaderStages::FRAGMENT);
        }
        stage
    }
}

struct BindEntryData {
    entry: Box<dyn BindEntry>,
    visibility: ShaderStage,
}

pub struct BindGroupBuilder {
    label: Option<String>,
    entries: Vec<BindEntryData>,
}
impl Default for BindGroupBuilder {
    fn default() -> Self {
        Self {
            label: None,
            entries: Vec::new(),
        }
    }
}
impl BindGroupBuilder {
    pub fn entry<T: BindEntry + Clone + 'static>(
        &mut self,
        visibility: ShaderStage,
        entry: &T,
    ) -> &mut Self {
        self.entries.push(BindEntryData {
            entry: Box::new(entry.clone()),
            visibility,
        });
        self
    }

    pub fn build(&self, gpu: &Gpu) -> BindGroup {
        let mut layout_entries = Vec::new();
        for (i, entry_data) in self.entries.iter().enumerate() {
            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: entry_data.visibility.into(),
                ty: entry_data.entry.ty(),
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
        for (i, entry_data) in self.entries.iter().enumerate() {
            entries.push(wgpu::BindGroupEntry {
                binding: i as u32,
                resource: entry_data.entry.resource(),
            });
        }

        let bind_group = gpu
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label.as_deref(),
                layout: &layout,
                entries: &entries,
            });

        BindGroup {
            handle: bind_group,
            layout,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BindGroup {
    handle: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
}
impl BindGroup {
    pub fn get_handle(&self) -> &wgpu::BindGroup {
        &self.handle
    }
    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}
