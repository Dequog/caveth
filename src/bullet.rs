use crate::enemy::Enemy;

use macroquad::prelude::*;

const BULLET_SPEED: f32 = 500.0;
const BULLET_SIZE: f32 = 25.0;

pub struct Bullet {
    pub rect: Rect,
    texture: Texture2D,
}

impl Bullet {
    pub async fn new(pos: Vec2) -> Self {
        Self {
            // We will use a rect to represent the bullet bounds
            rect: Rect::new(pos.x, pos.y, BULLET_SIZE, BULLET_SIZE),
            texture: load_texture("res/bullet.png").await.unwrap(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update the rect position
        self.rect.x += dt * BULLET_SPEED
    }

    pub fn draw(&self) {
        // Draw the bullet
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }

    pub fn is_collision(&self, enemy: &mut Enemy) -> bool {
        // If enemy collides with bullet
        if let Some(_intersection) = enemy.rect.intersect(self.rect) {
            return true;
        }

        false
    }
}
