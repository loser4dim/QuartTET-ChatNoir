pub struct MeshModel3D {
    pub vertices: Vec<[f32;3]>,
    pub indices : Vec<[u32;3]>
}

impl MeshModel3D {
    pub fn new() -> Self {
        let mut vertices = Vec::<[f32;3]>::new();
        vertices.push([1.0, 1.0, 1.0]);
        vertices.push([-1.0, -1.0, 1.0]);
        vertices.push([1.0, -1.0, -1.0]);
        vertices.push([-1.0, 1.0, -1.0]);

        let mut indices = Vec::<[u32;3]>::new();
        indices.push([0, 1, 2]);
        indices.push([1, 3, 2]);
        indices.push([0, 2, 3]);
        indices.push([0, 3, 1]);

        return Self{vertices, indices};
    }
}