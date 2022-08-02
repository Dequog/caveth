use macroquad::prelude::*;

pub struct Score {
    font: Font,
    pub score: i32,
    pub high_score: i32,
}

impl Score {
    pub async fn new() -> Self {
        Self {
            font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
            score: 0,
            high_score: 0,
        }
    }

    pub fn increment(&mut self) {
        // Increment the score
        self.score += 1;

        // Set the high score
        self.high_score = std::cmp::max(self.high_score, self.score);
    }

    pub fn reset(&mut self) {
        // set the high score
        self.high_score = std::cmp::max(self.high_score, self.score);

        // Reset the score
        self.score = 0;
    }

    pub fn draw(&mut self) {
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
