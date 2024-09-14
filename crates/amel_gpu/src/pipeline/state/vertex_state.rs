#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexAttributes {
    pub attributes: Vec<wgpu::VertexAttribute>,
    pub array_stride: wgpu::BufferAddress,
}

impl VertexAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_attribute(mut self, shader_location: u32, format: wgpu::VertexFormat) -> Self {
        self.attributes.push(wgpu::VertexAttribute {
            format,
            offset: self.array_stride,
            shader_location,
        });
        self.array_stride += format.size();
        self
    }

    pub fn vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
        }
    }
}

#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexBufferLayoutsBuilder {
    attributes: Vec<VertexAttributes>,
}

impl VertexBufferLayoutsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_attributes(&mut self, attributes: VertexAttributes) -> &mut Self {
        self.attributes.push(attributes);
        self
    }

    pub fn add_attributes_vec(&mut self, attributes: Vec<VertexAttributes>) -> &mut Self {
        for attr in attributes {
            self.add_attributes(attr);
        }
        self
    }

    pub fn build(&self) -> Vec<wgpu::VertexBufferLayout> {
        self.attributes
            .iter()
            .map(|attr| attr.vertex_buffer_layout())
            .collect()
    }
}

#[derive(Debug)]
pub struct VertexStateBuilder<'a> {
    shader: Option<&'a wgpu::ShaderModule>,
    entry_point: &'a str,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
    buffers: Option<&'a Vec<wgpu::VertexBufferLayout<'a>>>,
}

impl<'a> Default for VertexStateBuilder<'a> {
    fn default() -> Self {
        VertexStateBuilder {
            shader: None,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: None,
        }
    }
}

impl<'a> VertexStateBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shader(mut self, shader: &'a wgpu::ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn entry_point(mut self, entry_point: &'a str) -> Self {
        self.entry_point = entry_point;
        self
    }

    pub fn compilation_options(mut self, options: wgpu::PipelineCompilationOptions<'a>) -> Self {
        self.compilation_options = options;
        self
    }

    pub fn buffers(mut self, buffers: &'a Vec<wgpu::VertexBufferLayout<'a>>) -> Self {
        self.buffers = Some(buffers);
        self
    }

    pub fn build(self) -> wgpu::VertexState<'a> {
        wgpu::VertexState {
            module: self.shader.expect("Shader module is required"),
            entry_point: self.entry_point,
            compilation_options: self.compilation_options.clone(),
            buffers: self.buffers.unwrap(),
        }
    }
}
