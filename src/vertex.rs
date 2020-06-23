#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub const VERTEX_ATTRS: [wgpu::VertexAttributeDescriptor; 2] =
        wgpu::vertex_attr_array![0 => Float3, 1 => Float2];
    pub const VERTEX_BUFF_DESC: wgpu::VertexBufferDescriptor<'static> =
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRS,
        };

    pub const VERTICES: &'static [Vertex] = &[
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            tex_coords: [0.4131759, 0.00759614],
        }, // A
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            tex_coords: [0.0048659444, 0.43041354],
        }, // B
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            tex_coords: [0.28081453, 0.949397057],
        }, // C
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            tex_coords: [0.85967, 0.84732911],
        }, // D
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            tex_coords: [0.9414737, 0.2652641],
        }, // E
    ];

    pub const INDICES: &'static [u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}
