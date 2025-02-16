use crate::clock::MilliSecondClock32;
use crate::println;
use crate::snake_game::snake::Command;
use crate::snake_game::snake::Direction;
use crate::snake_game::snake::Point;
use crate::snake_game::snake::Snake;
use crate::vga_buffer;
use crate::vga_buffer::BUFFER_HEIGHT;
use crate::vga_buffer::BUFFER_WIDTH;
use core::time::Duration;
use embedded_timers::clock::Clock;
use embedded_timers::instant::Instant;
use nanorand::{Rng, WyRand};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

#[derive(Debug)]
pub struct Game {
    buffer_size: (u16, u16),
    width: u16,
    height: u16,
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    score: u16,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let buffer_size = (
            vga_buffer::BUFFER_WIDTH as u16,
            vga_buffer::BUFFER_HEIGHT as u16,
        );
        Self {
            buffer_size,
            width,
            height,
            food: None,
            snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
                match WyRand::new().generate_range(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    _ => Direction::Left,
                },
            ),
            speed: 0,
            score: 0,
        }
    }

    pub fn run(&mut self) {
        self.place_food();
        self.prepare_ui();
        self.render();

        let clock = MilliSecondClock32;

        let mut done = false;
        while !done {
            let interval = self.calculate_interval();
            let direction = self.snake.get_direction();
            let now = clock.now();

            while clock.elapsed(now) < interval {
                if let Some(command) = self.get_command(interval - clock.elapsed(now)) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                        Command::Turn(towards) => {
                            if direction != towards && direction.opposite() != towards {
                                self.snake.set_direction(towards);
                            }
                        }
                    }
                }
            }

            if self.has_collided_with_wall() || self.has_bitten_itself() {
                done = true;
            } else {
                self.snake.slither();

                if let Some(food_point) = self.food {
                    if self.snake.get_head_point() == food_point {
                        self.snake.grow();
                        self.place_food();
                        self.score += 1;

                        if self.score % ((self.width * self.height) / MAX_SPEED) == 0 {
                            self.speed += 1;
                        }
                    }
                }

                self.render();
            }
        }

        println!("Game Over! Your score is {}", self.score);
    }

    fn place_food(&mut self) {
        loop {
            let random_x = WyRand::new().generate_range(0..self.width);
            let random_y = WyRand::new().generate_range(0..self.height);
            let point = Point::new(random_x, random_y);
            if !self.snake.contains_point(&point) {
                self.food = Some(point);
                break;
            }
        }
    }

    fn prepare_ui(&mut self) {}

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64,
        )
    }

    fn get_command(&self, wait_for: Duration) -> Option<Command> {
        None
    }

    fn wait_for_key_event(&self, wait_for: Duration) -> ! {
        loop {}
    }

    fn has_collided_with_wall(&self) -> bool {
        let head_point = self.snake.get_head_point();

        match self.snake.get_direction() {
            Direction::Up => head_point.y == 0,
            Direction::Right => head_point.x == self.width - 1,
            Direction::Down => head_point.y == self.height - 1,
            Direction::Left => head_point.x == 0,
        }
    }

    fn has_bitten_itself(&self) -> bool {
        let next_head_point = self
            .snake
            .get_head_point()
            .transform(self.snake.get_direction(), 1);
        let mut next_body_points = self.snake.get_body_points().clone();
        next_body_points.remove(next_body_points.len() - 1);
        next_body_points.remove(0);

        next_body_points.contains(&next_head_point)
    }

    fn render(&self) {
        use vga_buffer::ScreenChar;
        use vga_buffer::{Color, ColorCode};
        use volatile::Volatile;

        vga_buffer::print_buffer(core::array::from_fn::<_, BUFFER_HEIGHT, _>(|_| {
            core::array::from_fn::<_, BUFFER_WIDTH, _>(|_| {
                Volatile::new(ScreenChar {
                    ascii_character: 0,
                    color_code: ColorCode::new(Color::Red, Color::White),
                })
            })
        }));
        todo!()
    }
}
