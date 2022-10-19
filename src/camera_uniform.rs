use cgmath::prelude::*;

use crate::camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub(crate) input_values: [f32; 4],
    pub(crate) view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            input_values: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(
        &mut self,
        camera: &camera::Camera,
        projection: &camera::Projection,
    ) {
        //let cam_pos: [f32; 4] = camera.position.to_homogeneous().into();
        self.input_values = [projection.aspect, 0.0, 0.0, 0.0];
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}
