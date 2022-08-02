use macroquad::prelude::*;

pub struct ExitButton {
    rect: Rect,
    text: String,
    pub is_pressed: bool,
    font: Font,
}

impl ExitButton {
    pub async fn new(text: String) -> Self {
        Self {
            rect: Rect::new(screen_width() * 2.5, 10.0, 100.0, 25.0),
            is_pressed: false,
            text,
            font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
        }
    }

    pub fn check_for_press(&mut self) {
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

    pub fn draw(&self) {
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
