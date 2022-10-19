use crate::model::ModelVertex;

pub(crate) struct Sprite {
    pub width: f32,
    pub height: f32,
    pub vertexes: Vec<ModelVertex>,
    pub indices: Vec<u16>,
}

impl Sprite {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            vertexes: vec![
                //Z forward
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, 0.0],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, 0.0],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, 0.0],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, 0.0],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
        }
    }
}
