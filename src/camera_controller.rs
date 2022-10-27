use cgmath::*;
use std::f32::consts::FRAC_PI_2;

use winit::dpi::PhysicalPosition;
use winit::event::*;

use crate::camera;
use crate::collision_detection::CollisionDetection;





const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub(crate) struct CameraController {
    pub press_left: bool,
    pub press_right: bool,
    pub press_forward: bool,
    pub press_backward: bool,
    press_up: bool,
    press_down: bool,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    pub mouse_pressed: bool,
    forward_vel: f32,
    backward_vel: f32,
    right_vel: f32,
    left_vel: f32,
    jump_vel: f32,
    on_floor: bool,
}

impl CameraController {
    pub(crate) fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            press_left: false,
            press_right: false,
            press_forward: false,
            press_backward: false,
            press_up: false,
            press_down: false,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            mouse_pressed: false,
            forward_vel: 0.0,
            backward_vel: 0.0,
            right_vel: 0.0,
            left_vel: 0.0,
            jump_vel: 0.0,
            on_floor: false,
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
                let press = state == &ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.press_forward = press;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.press_left = press;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.press_backward = press;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.press_right = press;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.press_up = press;
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
        collision: CollisionDetection,
    ) {
        let dt = dt as f32 * 0.001;

        self.movement(collision, camera, dt);

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

    fn movement(&mut self, collision: CollisionDetection, camera: &mut camera::Camera, dt: f32) {
        if collision.up {
            self.on_floor = true;
            self.jump_vel = 0.0;
        } else {
            self.on_floor = false;
        }
        if self.press_forward && self.forward_vel < 1.5 {
            self.forward_vel += 0.5;
        }
        if self.press_backward && self.backward_vel < 1.5 {
            self.backward_vel += 0.5;
        }
        if self.press_right && self.right_vel < 1.5 {
            self.right_vel += 0.5;
        }
        if self.press_left && self.left_vel < 1.5 {
            self.left_vel += 0.5;
        }
        if self.press_up && self.on_floor {
            self.on_floor = false;
            self.jump_vel += 2.0;
        }
        let move_in_x_forward = self.forward_vel * camera.yaw.cos() * self.speed * dt;
        if ((move_in_x_forward) < 0.0 && !collision.left)
            || ((move_in_x_forward) > 0.0 && !collision.right)
        {
            camera.position.x += move_in_x_forward;
        }
        let move_in_z_forward = self.forward_vel * camera.yaw.sin() * self.speed * dt;
        if ((move_in_z_forward) < 0.0 && !collision.forward)
            || ((move_in_z_forward) > 0.0 && !collision.backward)
        {
            camera.position.z += move_in_z_forward;
        }
        let move_in_x_backward = self.backward_vel * camera.yaw.cos() * self.speed * dt;
        if ((move_in_x_backward) > 0.0 && !collision.left)
            || ((move_in_x_backward) < 0.0 && !collision.right)
        {
            camera.position.x -= move_in_x_backward;
        }
        let move_in_z_backward = self.backward_vel * camera.yaw.sin() * self.speed * dt;
        if ((move_in_z_backward) > 0.0 && !collision.forward)
            || ((move_in_z_backward) < 0.0 && !collision.backward)
        {
            camera.position.z -= move_in_z_backward;
        }
        let move_in_x_right = self.right_vel * camera.yaw.sin() * self.speed * dt;
        if ((move_in_x_right) > 0.0 && !collision.left)
            || ((move_in_x_right) < 0.0 && !collision.right)
        {
            camera.position.x -= move_in_x_right;
        }
        let move_in_z_right = self.right_vel * camera.yaw.cos() * self.speed * dt;
        if ((move_in_z_right) < 0.0 && !collision.forward)
            || ((move_in_z_right) > 0.0 && !collision.backward)
        {
            camera.position.z += move_in_z_right;
        }
        let move_in_x_left = self.left_vel * camera.yaw.sin() * self.speed * dt;
        if ((move_in_x_left) < 0.0 && !collision.left)
            || ((move_in_x_left) > 0.0 && !collision.right)
        {
            camera.position.x += move_in_x_left;
        }
        let move_in_z_left = self.left_vel * camera.yaw.cos() * self.speed * dt;
        if ((move_in_z_left) > 0.0 && !collision.forward)
            || ((move_in_z_left) < 0.0 && !collision.backward)
        {
            camera.position.z -= move_in_z_left;
        }
        if self.forward_vel > 0.0 {
            self.forward_vel -= 0.1;
        }
        if self.backward_vel > 0.0 {
            self.backward_vel -= 0.1;
        }
        if self.right_vel > 0.0 {
            self.right_vel -= 0.1;
        }
        if self.left_vel > 0.0 {
            self.left_vel -= 0.1;
        }
        if !self.on_floor {
            self.jump_vel -= 0.05;
        }
        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        //camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;
        camera.position.y += self.jump_vel * self.speed * dt;
    }
}
