use crate::{camera, cube::Cube, floor::Floor, instance::Instance, slope::Slope};

pub(crate) struct CollisionDetection {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
    pub up: bool,
    pub down: bool,
}

impl CollisionDetection {
    pub(crate) fn new() -> Self {
        Self {
            left: false,
            right: false,
            forward: false,
            backward: false,
            up: false,
            down: false,
        }
    }

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

    pub fn slope_detect(
        &mut self,
        camera: &mut camera::Camera,
        instances: &[Instance],
        slope: &mut Slope,
    ) {
        if slope.orientation == 2 {
            for instance in instances {
                if (camera.position.x < (instance.position.x + slope.width + 0.2))
                    && (camera.position.x > (instance.position.x - (slope.width + 0.2)))
                    && (camera.position.z < (instance.position.z + slope.depth + 0.2))
                    && (camera.position.z > (instance.position.z - (slope.depth + 0.2)))
                    && (camera.position.y
                        < (instance.position.y
                            + (1.0
                                - (camera.position.z - instance.position.z)
                                    * (slope.height / slope.depth))
                            + 0.4))
                    && (camera.position.y
                        > (instance.position.y
                            + (1.0
                                - (camera.position.z - instance.position.z)
                                    * (slope.height / slope.depth))
                            - 0.4))
                {
                    camera.position.y = instance.position.y
                        + (1.0
                            - (camera.position.z - instance.position.z)
                                * (slope.height / slope.depth))
                        + 0.3;

                    self.up = true;
                    break;
                } else {
                    self.up = false;
                }
            }
        }
    }
}
