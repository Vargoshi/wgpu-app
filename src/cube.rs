use crate::model::ModelVertex;

pub(crate) struct Cube {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub vertexes: Vec<ModelVertex>,
    pub indices: Vec<u16>,
}

impl Cube {
    pub(crate) fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            width,
            height,
            depth,
            vertexes: vec![
                //Z forward
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, 1.0 * depth],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, 1.0 * depth],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, 1.0 * depth],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, 1.0 * depth],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                //Z backward
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, -1.0 * depth],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, -1.0 * depth],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, -1.0 * depth],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, -1.0 * depth],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                //X left
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, -1.0 * depth],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, -1.0 * depth],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, 1.0 * depth],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, 1.0 * depth],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                //X right
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, 1.0 * depth],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, 1.0 * depth],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, -1.0 * depth],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, -1.0 * depth],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                //Y top
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, -1.0 * depth],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, -1.0 * depth],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, 1.0 * height, 1.0 * depth],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, 1.0 * height, 1.0 * depth],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                //Y bottom
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, 1.0 * depth],
                    tex_coords: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, 1.0 * depth],
                    tex_coords: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [-1.0 * width, -1.0 * height, -1.0 * depth],
                    tex_coords: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                ModelVertex {
                    position: [1.0 * width, -1.0 * height, -1.0 * depth],
                    tex_coords: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
            ],
            indices: vec![
                0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12,
                16, 17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
            ],
        }
    }
}
