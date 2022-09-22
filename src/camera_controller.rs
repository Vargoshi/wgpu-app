use cgmath::*;
use std::f32::consts::FRAC_PI_2;

use winit::dpi::PhysicalPosition;
use winit::event::*;

use crate::camera;

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

    pub(crate) fn update_camera(&mut self, camera: &mut camera::Camera, dt: u128) {
        let dt = dt as f32 * 0.001;

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        //let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        //let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        //camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        //camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        let tiles = vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 0, 0, 0, 2, 2, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 2, 0,
            0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2,
            2, 2, 2, 2, 2, 2,
        ];
        let mapwidth = 8;

        dbg!(camera.position);
        dbg!(camera.yaw);

        let x_offset = if camera.yaw.cos() < 0.0 { -1 } else { 1 };

        let z_offset = if camera.yaw.sin() < 0.0 { -1 } else { 1 };

        let strafe_x_offset = if (camera.yaw + Rad(SAFE_FRAC_PI_2)).cos() < 0.0 {
            -1
        } else {
            1
        };

        let strafe_z_offset = if (camera.yaw + Rad(SAFE_FRAC_PI_2)).sin() < 0.0 {
            -1
        } else {
            1
        };

        let ipx = camera.position.x / 2.0;
        let ipx_add_xo = (camera.position.x as i32 + x_offset) / 2;
        let ipx_sub_xo = (camera.position.x as i32 - x_offset) / 2;
        let strafe_ipx_add_xo = (camera.position.x as i32 + strafe_x_offset) / 2;
        let strafe_ipx_sub_xo = (camera.position.x as i32 - strafe_x_offset) / 2;
        let ipz = camera.position.z / 2.0;
        let ipz_add_yo = (camera.position.z as i32 + z_offset) / 2;
        let ipz_sub_yo = (camera.position.z as i32 - z_offset) / 2;
        let strafe_ipz_add_yo = (camera.position.z as i32 + strafe_z_offset) / 2;
        let strafe_ipz_sub_yo = (camera.position.z as i32 - strafe_z_offset) / 2;

        if self.amount_forward > 0.0 {
            if tiles[(ipz as i32 * mapwidth + ipx_add_xo) as usize] == 0 {
                camera.position.x += self.amount_forward * camera.yaw.cos() * self.speed * dt;
            }

            if tiles[(ipz_add_yo * mapwidth + ipx as i32) as usize] == 0 {
                camera.position.z += self.amount_forward * camera.yaw.sin() * self.speed * dt;
            }
        }

        if self.amount_backward > 0.0 {
            if tiles[(ipz as i32 * mapwidth + ipx_sub_xo) as usize] == 0 {
                camera.position.x -= self.amount_backward * camera.yaw.cos() * self.speed * dt;
            }

            if tiles[(ipz_sub_yo * mapwidth + ipx as i32) as usize] == 0 {
                camera.position.z -= self.amount_backward * camera.yaw.sin() * self.speed * dt;
            }
        }

        if self.amount_right > 0.0 {
            if tiles[(ipz as i32 * mapwidth + strafe_ipx_add_xo) as usize] == 0 {
                camera.position.x -= self.amount_right * camera.yaw.sin() * self.speed * dt;
            }

            if tiles[(strafe_ipz_add_yo * mapwidth + ipx as i32) as usize] == 0 {
                camera.position.z += self.amount_right * camera.yaw.cos() * self.speed * dt;
            }
        }

        if self.amount_left > 0.0 {
            if tiles[(ipz as i32 * mapwidth + strafe_ipx_sub_xo) as usize] == 0 {
                camera.position.x += self.amount_left * camera.yaw.sin() * self.speed * dt;
            }

            if tiles[(strafe_ipz_sub_yo * mapwidth + ipx as i32) as usize] == 0 {
                camera.position.z -= self.amount_left * camera.yaw.cos() * self.speed * dt;
            }
        }

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward =
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

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
