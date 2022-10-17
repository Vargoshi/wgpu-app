use crate::{camera, instance::Instance};

pub(crate) struct CollisionDetection {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
    pub up: bool,
    pub down: bool,
}

impl CollisionDetection {
    pub fn detect(&mut self, camera: &mut camera::Camera, instances: &[Instance]) {
        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.5))
                && (camera.position.x > (instance.position.x + 1.0))
                && (camera.position.z < (instance.position.z + 1.3))
                && (camera.position.z > (instance.position.z - 1.3))
                && (camera.position.y < (instance.position.y + 1.3))
                && (camera.position.y > (instance.position.y - 1.3))
            {
                self.left = true;
                break;
            } else {
                self.left = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x - 1.0))
                && (camera.position.x > (instance.position.x - 1.5))
                && (camera.position.z < (instance.position.z + 1.3))
                && (camera.position.z > (instance.position.z - 1.3))
                && (camera.position.y < (instance.position.y + 1.3))
                && (camera.position.y > (instance.position.y - 1.3))
            {
                self.right = true;
                break;
            } else {
                self.right = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.3))
                && (camera.position.x > (instance.position.x - 1.3))
                && (camera.position.z < (instance.position.z + 1.5))
                && (camera.position.z > (instance.position.z + 1.0))
                && (camera.position.y < (instance.position.y + 1.3))
                && (camera.position.y > (instance.position.y - 1.3))
            {
                self.forward = true;
                break;
            } else {
                self.forward = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.3))
                && (camera.position.x > (instance.position.x - 1.3))
                && (camera.position.z < (instance.position.z - 1.0))
                && (camera.position.z > (instance.position.z - 1.5))
                && (camera.position.y < (instance.position.y + 1.3))
                && (camera.position.y > (instance.position.y - 1.3))
            {
                self.backward = true;
                break;
            } else {
                self.backward = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.3))
                && (camera.position.x > (instance.position.x - 1.3))
                && (camera.position.z < (instance.position.z + 1.3))
                && (camera.position.z > (instance.position.z - 1.3))
                && (camera.position.y < (instance.position.y + 2.0))
                && (camera.position.y > (instance.position.y + 1.5))
            {
                self.up = true;
                break;
            } else {
                self.up = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + 1.3))
                && (camera.position.x > (instance.position.x - 1.3))
                && (camera.position.z < (instance.position.z + 1.3))
                && (camera.position.z > (instance.position.z - 1.3))
                && (camera.position.y < (instance.position.y - 1.0))
                && (camera.position.y > (instance.position.y - 1.5))
            {
                self.down = true;
                break;
            } else {
                self.down = false;
            }
        }
    }
}
