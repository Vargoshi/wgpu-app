use crate::model::ModelVertex;

pub(crate) struct Slope {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub vertexes: Vec<ModelVertex>,
    pub indices: Vec<u16>,
    pub orientation: i32,
}

impl Slope {
    pub(crate) fn new(width: f32, height: f32, depth: f32, orientation: &str) -> Self {
        if orientation == "forward" {
            Self {
                width,
                height,
                depth,
                vertexes: vec![
                    ModelVertex {
                        position: [1.0 * width, -1.0 * height, -1.0 * depth],
                        tex_coords: [1.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [-1.0 * width, -1.0 * height, -1.0 * depth],
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
                ],
                indices: vec![0, 1, 2, 2, 3, 0],
                orientation: 1,
            }
        } else if orientation == "backward" {
            Self {
                width,
                height,
                depth,
                vertexes: vec![
                    ModelVertex {
                        position: [1.0 * width, 1.0 * height, -1.0 * depth],
                        tex_coords: [1.0 * width, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [-1.0 * width, 1.0 * height, -1.0 * depth],
                        tex_coords: [0.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [-1.0 * width, -1.0 * height, 1.0 * depth],
                        tex_coords: [0.0, 1.0 * height],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [1.0 * width, -1.0 * height, 1.0 * depth],
                        tex_coords: [1.0 * width, 1.0 * height],
                        normal: [0.0, 0.0, 0.0],
                    },
                ],
                indices: vec![0, 1, 2, 2, 3, 0],
                orientation: 2,
            }
        } else if orientation == "left" {
            Self {
                width,
                height,
                depth,
                vertexes: vec![
                    ModelVertex {
                        position: [1.0 * width, -1.0 * height, -1.0 * depth],
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
                        position: [1.0 * width, -1.0 * height, 1.0 * depth],
                        tex_coords: [1.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                ],
                indices: vec![0, 1, 2, 2, 3, 0],
                orientation: 3,
            }
        } else if orientation == "right" {
            Self {
                width,
                height,
                depth,
                vertexes: vec![
                    ModelVertex {
                        position: [1.0 * width, 1.0 * height, -1.0 * depth],
                        tex_coords: [1.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [-1.0 * width, -1.0 * height, -1.0 * depth],
                        tex_coords: [0.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [-1.0 * width, -1.0 * height, 1.0 * depth],
                        tex_coords: [0.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [1.0 * width, 1.0 * height, 1.0 * depth],
                        tex_coords: [1.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                ],
                indices: vec![0, 1, 2, 2, 3, 0],
                orientation: 4,
            }
        } else {
            Self {
                width,
                height,
                depth,
                vertexes: vec![
                    ModelVertex {
                        position: [1.0 * width, -1.0 * height, -1.0 * depth],
                        tex_coords: [1.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    ModelVertex {
                        position: [-1.0 * width, -1.0 * height, -1.0 * depth],
                        tex_coords: [0.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
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
                ],
                indices: vec![0, 1, 2, 2, 3, 0],
                orientation: 0,
            }
        }
    }
}
