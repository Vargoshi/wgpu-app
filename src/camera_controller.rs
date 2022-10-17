use cgmath::*;
use std::f32::consts::FRAC_PI_2;

use winit::dpi::PhysicalPosition;
use winit::event::*;

use crate::camera;
use crate::collision_detection::CollisionDetection;
use crate::instance::Instance;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub(crate) struct CameraController {
    pub amount_left: f32,
    pub amount_right: f32,
    pub amount_forward: f32,
    pub amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    pub mouse_pressed: bool,
}

impl CameraController {
    pub(crate) fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            mouse_pressed: false,
        }
    }

    pub(crate) fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let amount = if state == &ElementState::Pressed {
                    1.0
                } else {
                    0.0
                };
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.amount_forward = amount;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.amount_left = amount;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.amount_backward = amount;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.amount_right = amount;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.amount_up = amount;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.amount_down = amount;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    pub(crate) fn update_camera(
        &mut self,
        camera: &mut camera::Camera,
        dt: u128,
        instances: &[Instance],
    ) {
        let dt = dt as f32 * 0.001;

        let mut collision = CollisionDetection {
            left: 0.0,
            right: 0.0,
            forward: 0.0,
            backward: 0.0,
            up: 0.0,
            down: 0.0,
        };

        collision.detect(camera, instances);

        let move_in_x_forward = self.amount_forward * camera.yaw.cos() * self.speed * dt;

        if ((move_in_x_forward) < 0.0 && collision.left < 1.0)
            || ((move_in_x_forward) > 0.0 && collision.right < 1.0)
        {
            camera.position.x += move_in_x_forward;
        }

        let move_in_z_forward = self.amount_forward * camera.yaw.sin() * self.speed * dt;

        if ((move_in_z_forward) < 0.0 && collision.forward < 1.0)
            || ((move_in_z_forward) > 0.0 && collision.backward < 1.0)
        {
            camera.position.z += move_in_z_forward;
        }

        let move_in_x_backward = self.amount_backward * camera.yaw.cos() * self.speed * dt;

        if ((move_in_x_backward) > 0.0 && collision.left < 1.0)
            || ((move_in_x_backward) < 0.0 && collision.right < 1.0)
        {
            camera.position.x -= move_in_x_backward;
        }

        let move_in_z_backward = self.amount_backward * camera.yaw.sin() * self.speed * dt;

        if ((move_in_z_backward) > 0.0 && collision.forward < 1.0)
            || ((move_in_z_backward) < 0.0 && collision.backward < 1.0)
        {
            camera.position.z -= move_in_z_backward;
        }

        let move_in_x_right = self.amount_right * camera.yaw.sin() * self.speed * dt;

        if ((move_in_x_right) > 0.0 && collision.left < 1.0)
            || ((move_in_x_right) < 0.0 && collision.right < 1.0)
        {
            camera.position.x -= move_in_x_right;
        }

        let move_in_z_right = self.amount_right * camera.yaw.cos() * self.speed * dt;

        if ((move_in_z_right) < 0.0 && collision.forward < 1.0)
            || ((move_in_z_right) > 0.0 && collision.backward < 1.0)
        {
            camera.position.z += move_in_z_right;
        }

        let move_in_x_left = self.amount_left * camera.yaw.sin() * self.speed * dt;

        if ((move_in_x_left) < 0.0 && collision.left < 1.0)
            || ((move_in_x_left) > 0.0 && collision.right < 1.0)
        {
            camera.position.x += move_in_x_left;
        }

        let move_in_z_left = self.amount_left * camera.yaw.cos() * self.speed * dt;

        if ((move_in_z_left) > 0.0 && collision.forward < 1.0)
            || ((move_in_z_left) < 0.0 && collision.backward < 1.0)
        {
            camera.position.z -= move_in_z_left;
        }

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }

        if camera.yaw < -Rad(SAFE_FRAC_PI_2 * 2.0) {
            camera.yaw += Rad(SAFE_FRAC_PI_2 * 4.0);
        } else if camera.yaw > Rad(SAFE_FRAC_PI_2 * 2.0) {
            camera.yaw -= Rad(SAFE_FRAC_PI_2 * 4.0);
        }
    }
}
