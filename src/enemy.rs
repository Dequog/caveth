use macroquad::prelude::*;

const ENEMY_SIZE: f32 = 50.0;
const ENEMY_SPEED: f32 = 500.0;

pub struct Enemy {
    pub rect: Rect,
    texture: Texture2D,
    pub is_alive: bool,
    pub has_given_points: bool,
}

impl Enemy {
    pub async fn new(pos: Vec2) -> Self {
        Self {
            // We will use a rect to represent the enemy bounds
            rect: Rect::new(pos.x, pos.y, ENEMY_SIZE, ENEMY_SIZE),
            texture: load_texture("res/enemy.png").await.unwrap(),
            is_alive: true,
            has_given_points: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_alive {
            // Update the rect position
            self.rect.x -= dt * ENEMY_SPEED;
        }
    }

    pub fn draw(&self) {
        if self.is_alive {
            // Draw the enemy
            draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
        }
    }
}
