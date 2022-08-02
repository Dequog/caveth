mod bullet;
mod enemy;
mod exit_button;
mod player;
mod score;

use bullet::Bullet;
use enemy::Enemy;
use exit_button::ExitButton;
use player::Player;
use score::Score;

use macroquad::prelude::*;

const BULLET_COOLDOWN_TIME: f32 = 5.0;

const ENEMY_SPAWN_TIME: f32 = 50.0;

enum GameState {
    Menu,
    Playing,
    GameOver,
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
                        if bullet.is_collision(enemy) {
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
                    if enemy.rect.x < 0.0 - player.rect.w || player.is_collision(enemy) {
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
