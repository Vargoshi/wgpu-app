use crate::{camera, cube::Cube, floor::Floor, instance::Instance};

pub(crate) struct CollisionDetection {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
    pub up: bool,
    pub down: bool,
}

impl CollisionDetection {
    pub fn detect(&mut self, camera: &mut camera::Camera, instances: &[Instance], cube: &mut Cube) {
        for instance in instances {
            if (camera.position.x < (instance.position.x + cube.width + 0.5))
                && (camera.position.x > (instance.position.x + cube.width))
                && (camera.position.z < (instance.position.z + cube.depth + 0.3))
                && (camera.position.z > (instance.position.z - (cube.depth + 0.3)))
                && (camera.position.y < (instance.position.y + cube.height + 0.7))
                && (camera.position.y > (instance.position.y - (cube.height + 0.7)))
            {
                self.left = true;
                break;
            } else {
                self.left = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x - cube.width))
                && (camera.position.x > (instance.position.x - (cube.width + 0.5)))
                && (camera.position.z < (instance.position.z + cube.depth + 0.3))
                && (camera.position.z > (instance.position.z - (cube.depth + 0.3)))
                && (camera.position.y < (instance.position.y + cube.height + 0.7))
                && (camera.position.y > (instance.position.y - (cube.height + 0.7)))
            {
                self.right = true;
                break;
            } else {
                self.right = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + cube.width + 0.3))
                && (camera.position.x > (instance.position.x - (cube.width + 0.3)))
                && (camera.position.z < (instance.position.z + cube.depth + 0.5))
                && (camera.position.z > (instance.position.z + cube.depth))
                && (camera.position.y < (instance.position.y + cube.height + 0.7))
                && (camera.position.y > (instance.position.y - (cube.height + 0.7)))
            {
                self.forward = true;
                break;
            } else {
                self.forward = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + cube.width + 0.3))
                && (camera.position.x > (instance.position.x - (cube.width + 0.3)))
                && (camera.position.z < (instance.position.z - cube.depth))
                && (camera.position.z > (instance.position.z - (cube.depth + 0.5)))
                && (camera.position.y < (instance.position.y + cube.height + 0.7))
                && (camera.position.y > (instance.position.y - (cube.height + 0.7)))
            {
                self.backward = true;
                break;
            } else {
                self.backward = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + cube.width + 0.3))
                && (camera.position.x > (instance.position.x - (cube.width + 0.3)))
                && (camera.position.z < (instance.position.z + cube.depth + 0.3))
                && (camera.position.z > (instance.position.z - (cube.depth + 0.3)))
                && (camera.position.y < (instance.position.y + cube.height + 1.0))
                && (camera.position.y > (instance.position.y + cube.height + 0.5))
            {
                self.up = true;
                break;
            } else {
                self.up = false;
            }
        }

        for instance in instances {
            if (camera.position.x < (instance.position.x + cube.width + 0.3))
                && (camera.position.x > (instance.position.x - (cube.width + 0.3)))
                && (camera.position.z < (instance.position.z + cube.depth + 0.3))
                && (camera.position.z > (instance.position.z - (cube.depth + 0.3)))
                && (camera.position.y < (instance.position.y - cube.height))
                && (camera.position.y > (instance.position.y - (cube.height + 0.5)))
            {
                self.down = true;
                break;
            } else {
                self.down = false;
            }
        }
    }

    pub fn floor_detect(
        &mut self,
        camera: &mut camera::Camera,
        instances: &[Instance],
        floor: &mut Floor,
    ) {
        for instance in instances {
            if (camera.position.x < (instance.position.x + floor.width + 0.3))
                && (camera.position.x > (instance.position.x - (floor.width + 0.3)))
                && (camera.position.z < (instance.position.z + floor.depth + 0.3))
                && (camera.position.z > (instance.position.z - (floor.depth + 0.3)))
                && (camera.position.y < (instance.position.y))
                && (camera.position.y > (instance.position.y - (floor.height / 2.0)))
            {
                self.up = true;
                break;
            } else {
                self.up = false;
            }
        }
    }
}
