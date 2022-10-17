use crate::{camera, instance::Instance};

pub(crate) struct CollisionDetection {
    pub left: f32,
    pub right: f32,
    pub forward: f32,
    pub backward: f32,
    pub up: f32,
    pub down: f32,
}

impl CollisionDetection {
    pub fn detect(&mut self, camera: &mut camera::Camera, instances: &[Instance]) {
        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.5))
                && (camera.position.x > (instance.position.x + 1.0))
                && (camera.position.z < (instance.position.z + 1.5))
                && (camera.position.z > (instance.position.z - 1.5))
            {
                self.left = 1.0;
                break;
            } else {
                self.left = 0.0;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x - 1.0))
                && (camera.position.x > (instance.position.x - 1.5))
                && (camera.position.z < (instance.position.z + 1.5))
                && (camera.position.z > (instance.position.z - 1.5))
            {
                self.right = 1.0;
                break;
            } else {
                self.right = 0.0;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.5))
                && (camera.position.x > (instance.position.x - 1.5))
                && (camera.position.z < (instance.position.z + 1.5))
                && (camera.position.z > (instance.position.z + 1.0))
            {
                self.forward = 1.0;
                break;
            } else {
                self.forward = 0.0;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.5))
                && (camera.position.x > (instance.position.x - 1.5))
                && (camera.position.z < (instance.position.z - 1.0))
                && (camera.position.z > (instance.position.z - 1.5))
            {
                self.backward = 1.0;
                break;
            } else {
                self.backward = 0.0;
            }
        }
    }
}
