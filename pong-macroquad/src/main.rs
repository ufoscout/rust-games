use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

use constants::*;

mod constants;

fn conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut paddle_left = Paddle::new(Rect::new(
        PADDLE_W,
        screen_height() / 2.,
        PADDLE_W,
        PADDLE_H,
    ));
    let mut paddle_right = Paddle::new(Rect::new(
        screen_width() - PADDLE_W * 2.,
        screen_height() / 2.,
        PADDLE_W,
        PADDLE_H,
    ));
    let mut ball = Ball::new(Circle::new(
        screen_width() / 2.,
        screen_height() / 2.,
        BALL_RADIUS,
    ));
    let mut scores: (u32, u32) = (0, 0);

    let font = load_ttf_font("./assets/Roboto/Roboto-Black.ttf").await.unwrap();
    loop {
        clear_background(SKYBLUE);
        paddle_left.movement(KeyCode::W, KeyCode::S);
        paddle_right.movement(KeyCode::Up, KeyCode::Down);
        ball.move_ball();
        ball.collision_with_paddle(&paddle_left.rect, &paddle_right.rect);
        ball.draw();
        paddle_left.draw();
        paddle_right.draw();

        if ball.circle.x < 0.0 {
            ball = Ball::new(Circle::new(
                screen_width() / 2.,
                screen_height() / 2.,
                BALL_RADIUS,
            ));
            scores.1 += 1;
        } else if ball.circle.x > screen_width() {
            ball = Ball::new(Circle::new(
                screen_width() / 2.,
                screen_height() / 2.,
                BALL_RADIUS,
            ));
            scores.0 += 1;
        }
        draw_scores(&font, &scores);
        next_frame().await;
    }
}

#[derive(Copy, Clone)]
struct Paddle {
    rect: Rect,
}

impl Paddle {
    fn new(rect: Rect) -> Self {
        Self { rect }
    }

    fn movement(&mut self, up: KeyCode, down: KeyCode) {
        if is_key_down(up) {
            self.rect.y -= 1. * PADDLE_SPEED;
        } else if is_key_down(down) {
            self.rect.y += 1. * PADDLE_SPEED;
        }

        if self.rect.y > screen_height() - PADDLE_H {
            self.rect.y = screen_height() - PADDLE_H;
        } else if self.rect.y < 0. {
            self.rect.y = 0.;
        }
    }

    fn draw(&self) {
        let r = self.rect;
        draw_rectangle(r.x, r.y, r.w, r.h, PADDLE_COLOR);
    }
}

#[derive(Copy, Clone)]
struct Ball {
    circle: Circle,
    dir: Vec2,
}

impl Ball {
    fn new(circle: Circle) -> Self {
        let mut rng = thread_rng();
        let dir_x = rng.gen_range(-1.0..1.);
        let dir_y = rng.gen_range(-1.0..1.);
        Self {
            circle,
            dir: vec2(dir_x, dir_y),
        }
    }

    fn move_ball(&mut self) {
        self.circle.x += self.dir.x * BALL_SPEED;
        self.circle.y += self.dir.y * BALL_SPEED;

        if self.circle.y > screen_height() - BALL_RADIUS || self.circle.y < 0.0 {
            self.dir.y = -self.dir.y;
        }
    }

    fn collision_with_paddle(&mut self, paddle_1: &Rect, paddle_2: &Rect) {
        let ball_rect = Rect::new(self.circle.x, self.circle.y, self.circle.r, self.circle.r);
        if ball_rect.intersect(*paddle_1).is_some() || ball_rect.intersect(*paddle_2).is_some() {
            self.dir.x = -self.dir.x;
        }
    }

    fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, BALL_COLOR);
    }
}

fn draw_scores(font: &Font, scores: &(u32, u32)) {
    let text_params = TextParams {
        font_size: 70,
        font: Some(font),
        ..Default::default()
    };

    draw_text_ex(
        &format!("{}", scores.0).as_str(),
        100.,
        100.,
        text_params.clone(),
    );
    draw_text_ex(
        &format!("{}", scores.1).as_str(),
        screen_width() - 100.,
        100.,
        text_params,
    );
}
