use super::Buffer;

pub struct BufferBuilder<'a> {
    usages: wgpu::BufferUsages,
    label: wgpu::Label<'a>,
}

impl<'a> Default for BufferBuilder<'a> {
    fn default() -> Self {
        Self {
            usages: wgpu::BufferUsages::empty(),
            label: None,
        }
    }
}

impl<'a> BufferBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn label(mut self, label: impl Into<wgpu::Label<'a>>) -> Self {
        self.label = label.into();
        self
    }

    #[inline]
    pub fn vertex(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::VERTEX;
        self
    }

    #[inline]
    pub fn index(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::INDEX;
        self
    }

    #[inline]
    pub fn storage(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::STORAGE;
        self
    }

    #[inline]
    pub fn uniform(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::UNIFORM;
        self
    }

    #[inline]
    pub fn copy_dst(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::COPY_DST;
        self
    }

    #[inline]
    pub fn copy_src(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::COPY_SRC;
        self
    }

    #[inline]
    pub fn read(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::MAP_READ;
        self
    }

    #[inline]
    pub fn write(mut self) -> Self {
        self.usages |= wgpu::BufferUsages::MAP_WRITE;
        self
    }

    pub fn build(&self, device: &wgpu::Device, item_size: usize, item_count: usize) -> Buffer {
        Buffer::new(device, self.usages, item_size, item_count)
    }

    pub fn build_with_data(&self, device: &wgpu::Device, item_size: usize, bytes: &[u8]) -> Buffer {
        Buffer::new_with_data(device, self.usages, item_size, bytes)
    }
}
