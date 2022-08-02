use crate::enemy::Enemy;

use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([80.0, 44.0]);
const PLAYER_SPEED: f32 = 500.0;

pub struct Player {
    pub rect: Rect,
    texture: Texture2D,
    pub can_shoot: bool,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            // We are using a rect to represent to player bounds
            rect: Rect::new(
                0.0,
                screen_height() * 0.5 - PLAYER_SIZE[1] * 2.0,
                PLAYER_SIZE[0],
                PLAYER_SIZE[1],
            ),
            texture: load_texture("res/player.png").await.unwrap(),
            can_shoot: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let vel = match (
            is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::K),
            is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) || is_key_down(KeyCode::K),
        ) {
            // If the down arrow, W, or K is pressed set velocity to -1
            (true, false) => -1.0,

            // If the down arrow, S, or J is pressed set velocity to 1
            (false, true) => 1.0,
            _ => 0.0,
        };

        // Update the rect position
        self.rect.y += vel * dt * PLAYER_SPEED;

        // Check if the rect is out of bounds

        // Top bound
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        }

        // Bottom bound
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    pub fn draw(&self) {
        // Draw the player
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }

    pub fn reset(&mut self) {
        self.can_shoot = true;

        // Reset the rect position
        self.rect.y = screen_height() * 0.5 - PLAYER_SIZE[1] * 2.0;
    }

    pub fn is_collision(&self, enemy: &mut Enemy) -> bool {
        // If enemy collides with bullet
        if let Some(_intersection) = enemy.rect.intersect(self.rect) {
            return true;
        }

        false
    }
}
