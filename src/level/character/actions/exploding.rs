use super::super::sprite::Sprite;
use super::super::update_result::UpdateResult;
use crate::screen::Buffer;
use image::Rgba;
use std::f32::consts::PI;

const ANIMATION_LENGTH: i32 = 40;
const RADIUS: f32 = 15.0;
const SPEED_MIN: f32 = 1.0;
const SPEED_MAX: f32 = 8.0;
const PARTICLE_COUNT: usize = 300;

pub struct Exploding {
    sprite: Sprite,
    frame_index: i32,
    particles: Vec<Particle>,
}

struct Particle {
    position: Vec2d,
    velocity: Vec2d,
}

struct Vec2d {
    x: f32,
    y: f32,
}

impl Exploding {
    pub fn new(sprite: Sprite) -> Self {
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);

        for _ in 0..PARTICLE_COUNT {
            let angle = fastrand::f32() * PI * 0.75 + PI * 0.125;

            let distance = fastrand::f32() * RADIUS;
            let x = sprite.x as f32 + (angle.cos() * distance);
            let y = sprite.y as f32 - (angle.sin() * distance);

            let speed = fastrand::f32() * (SPEED_MAX - SPEED_MIN) + SPEED_MIN;
            let vx = angle.cos() * speed;
            let vy = -(angle.sin() * speed);

            particles.push(Particle {
                position: Vec2d { x, y },
                velocity: Vec2d { x: vx, y: vy },
            });
        }

        Exploding {
            sprite,
            frame_index: 0,
            particles,
        }
    }

    pub fn update(&mut self) -> Option<UpdateResult> {
        if self.frame_index >= ANIMATION_LENGTH {
            return Some(UpdateResult::Dead);
        }

        self.frame_index += 1;

        for particle in &mut self.particles {
            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;
            particle.velocity.y += 0.3;
        }

        None
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        let alpha = 255 - (self.frame_index * 255 / ANIMATION_LENGTH) as u8;
        let color = Rgba([255, 0, 0, alpha]);

        for particle in &self.particles {
            buffer.set_pixel(
                particle.position.x as i32,
                particle.position.y as i32,
                color,
            );
        }
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
