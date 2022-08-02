use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([80.0, 44.0]);
const PLAYER_SPEED: f32 = 500.0;

const BULLET_SPEED: f32 = 500.0;
const BULLET_SIZE: f32 = 25.0;
const BULLET_COOLDOWN_TIME: f32 = 5.0;

const ENEMY_SIZE: f32 = 50.0;
const ENEMY_SPEED: f32 = 500.0;
const ENEMY_SPAWN_TIME: f32 = 50.0;

enum GameState {
    Menu,
    Playing,
    GameOver,
}

struct Player {
    rect: Rect,
    texture: Texture2D,
    can_shoot: bool,
}

impl Player {
    async fn new() -> Self {
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

    fn update(&mut self, dt: f32) {
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

        // Left side
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        }

        // Right side
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    fn draw(&self) {
        // Draw the player
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }

    fn reset(&mut self) {
        self.can_shoot = true;

        // Reset the rect position
        self.rect.y = screen_height() * 0.5 - PLAYER_SIZE[1] * 2.0;
    }
}

struct Bullet {
    rect: Rect,
    texture: Texture2D,
}

impl Bullet {
    async fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BULLET_SIZE, BULLET_SIZE),
            texture: load_texture("res/bullet.png").await.unwrap(),
        }
    }

    fn update(&mut self, dt: f32) {
        // Update the rect position
        self.rect.x += dt * BULLET_SPEED
    }

    fn draw(&self) {
        // Draw the bullet
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }
}

struct Enemy {
    rect: Rect,
    texture: Texture2D,
    is_alive: bool,
    has_given_points: bool,
}

impl Enemy {
    async fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, ENEMY_SIZE, ENEMY_SIZE),
            texture: load_texture("res/enemy.png").await.unwrap(),
            is_alive: true,
            has_given_points: false,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.is_alive {
            // Update the rect position
            self.rect.x -= dt * ENEMY_SPEED;
        }
    }

    fn draw(&self) {
        if self.is_alive {
            // Draw the enemy
            draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
        }
    }
}

struct Score {
    font: Font,
    score: i32,
    high_score: i32,
}

impl Score {
    async fn new() -> Self {
        Self {
            font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
            score: 0,
            high_score: 0,
        }
    }

    fn increment(&mut self) {
        // Increment the score
        self.score += 1;

        // Set the high score
        self.high_score = std::cmp::max(self.high_score, self.score);
    }

    fn reset(&mut self) {
        // set the high score
        self.high_score = std::cmp::max(self.high_score, self.score);

        // Reset the score
        self.score = 0;
    }

    fn draw(&mut self) {
        // Draw the Score
        draw_text_ex(
            self.score.to_string().as_str(),
            screen_width() * 0.5 - 7.5 - self.score.to_string().chars().count() as f32 * 7.5,
            40.0,
            TextParams {
                font: self.font,
                font_size: 30,
                color: BLACK,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
            },
        );
    }
}

fn is_collision(a: &mut Rect, b: &mut Rect) -> bool {
    // If a collides with b
    if let Some(_intersection) = a.intersect(*b) {
        return true;
    }

    false
}

struct ExitButton {
    rect: Rect,
    text: String,
    is_pressed: bool,
    font: Font,
}

impl ExitButton {
    async fn new(text: String) -> Self {
        Self {
            rect: Rect::new(screen_width() * 2.5, 10.0, 100.0, 25.0),
            is_pressed: false,
            text,
            font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
        }
    }

    fn check_for_press(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            // Check if the cursor is in the button area
            if self
                .rect
                .contains(vec2(mouse_position().0, mouse_position().1))
            {
                self.is_pressed = true;
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            self.is_pressed = false;
        }
    }

    fn draw(&self) {
        // Draw button background
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);

        // Draw button text
        draw_text_ex(
            self.text.as_str(),
            self.rect.x + self.rect.w * 0.5 - (self.text.chars().count() - 1) as f32 * 7.5 * 0.5,
            self.rect.y + self.rect.h * 0.5 + 7.5,
            TextParams {
                font: self.font,
                font_size: 15,
                color: BLACK,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
            },
        );
    }
}

#[macroquad::main("Caveth")]
async fn main() {
    let mut player = Player::new().await;
    let mut exit_button = ExitButton::new("Hold to Quit".to_string()).await;
    let mut score = Score::new().await;

    // There will be a lot of bullets and enemies, so we put them in a vector
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();

    let mut enemy_spawn_timer = ENEMY_SPAWN_TIME;

    let mut bullet_cooldown_timer = BULLET_COOLDOWN_TIME;

    let mut game_state = GameState::Menu;

    loop {
        clear_background(WHITE);

        if exit_button.is_pressed || is_key_down(KeyCode::Escape) {
            // Exit program
            std::process::exit(0);
        }

        match game_state {
            GameState::Menu => {
                if is_key_down(KeyCode::Space) {
                    // Start game
                    game_state = GameState::Playing;
                }
                // Draw start instructions
                draw_text_ex(
                    "Hold space to start",
                    screen_width() * 0.5 - 140.0,
                    screen_height() * 0.5,
                    TextParams {
                        font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
                        font_size: 30,
                        color: BLACK,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                    },
                );
            }
            GameState::Playing => {
                if bullet_cooldown_timer > 0.0 {
                    bullet_cooldown_timer -= 1.0;
                }

                if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) {
                    // Check if the cooldown timer is up and the mouse button/space has been released (to prevent spam)
                    if bullet_cooldown_timer == 0.0 && player.can_shoot {
                        // Spawn new bullet
                        bullets.push(
                            Bullet::new(player.rect.point() + player.rect.size() * 0.5).await,
                        );

                        // Reset cooldown timer
                        bullet_cooldown_timer = BULLET_COOLDOWN_TIME;

                        // Stop player from shooting until mouse button/space has been released
                        player.can_shoot = false;
                    }
                }

                if is_mouse_button_released(MouseButton::Left) || is_key_released(KeyCode::Space) {
                    // Let the player shoot again
                    player.can_shoot = true;
                }

                if enemy_spawn_timer > 0.0 {
                    enemy_spawn_timer -= 1.0;
                } else {
                    // Spawn enemy
                    enemies.push(
                        Enemy::new(vec2(
                            screen_width(),
                            rand::gen_range(100.0, screen_height() - 100.0),
                        ))
                        .await,
                    );

                    // Reset enemy spawn timer
                    enemy_spawn_timer = ENEMY_SPAWN_TIME;
                }

                // Draw and update bullets
                for bullet in &mut bullets {
                    bullet.draw();
                    bullet.update(get_frame_time());
                }

                for enemy in &mut enemies {
                    // Check if a bullet has collided with the enemy
                    for bullet in &mut bullets {
                        if is_collision(&mut bullet.rect, &mut enemy.rect) {
                            enemy.is_alive = false;

                            if !enemy.has_given_points {
                                score.increment();
                                enemy.has_given_points = true;
                            }
                        }
                    }

                    // Draw and update enemies
                    enemy.draw();
                    enemy.update(get_frame_time());

                    // Check if enemy is reached the end of the screen or has touched player
                    if enemy.rect.x < 0.0 - player.rect.w
                        || is_collision(&mut player.rect, &mut enemy.rect)
                    {
                        game_state = GameState::GameOver;
                    }
                }

                player.update(get_frame_time());
            }
            GameState::GameOver => {
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Playing;
                    score.reset();
                }

                // Draw high score
                draw_text_ex(
                    format!("High Score: {} ", score.high_score).as_str(),
                    screen_width() * 0.5 - 175.0,
                    screen_height() * 0.5 - 100.0,
                    TextParams {
                        font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
                        font_size: 30,
                        color: BLACK,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                    },
                );

                // Draw score
                draw_text_ex(
                    format!("Score: {} ", score.score).as_str(),
                    screen_width() * 0.5 - 175.0,
                    screen_height() * 0.5 - 50.0,
                    TextParams {
                        font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
                        font_size: 30,
                        color: BLACK,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                    },
                );

                // Draw retry instructions
                draw_text_ex(
                    "Hold space to try again",
                    screen_width() * 0.5 - 175.0,
                    screen_height() * 0.5,
                    TextParams {
                        font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
                        font_size: 30,
                        color: BLACK,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                    },
                );

                // Reset game (except for score)
                player.reset();
                bullets.clear();
                enemies.clear();

                // Reset spawn/cooldown timers
                enemy_spawn_timer = ENEMY_SPAWN_TIME;
                bullet_cooldown_timer = BULLET_COOLDOWN_TIME;
            }
        }

        // Check the exit button and check if it has been pressed
        exit_button.draw();
        exit_button.check_for_press();

        player.draw();
        score.draw();

        next_frame().await
    }
}
