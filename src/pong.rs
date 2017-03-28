// pong.rs
//
// The logic for pong

use matrix;

const PLAYER_HEIGHT: u32 = 2;

pub struct Pong {
    ball_x: u32,
    ball_y: u32,

    /// The number of left or right leds required before moving up or down
    ball_counts: u32,
    ball_current_count: u32,

    /// The x direction the ball it going, true => (+), false => (-)
    ball_direction_x: bool,

    /// The y direction the ball it going, true => (+), false => (-)
    ball_direction_y: bool,

    left_player: u32,
    right_player: u32,

    last_update: u32,
    delay: u32
}

impl Pong {
    pub fn new(delay: u32) -> Pong {
        Pong {
            ball_x: (matrix::WIDTH as u32)/2,
            ball_y: (matrix::HEIGHT as u32)/2,

            ball_counts: 2,
            ball_current_count: 0,

            ball_direction_x: true,
            ball_direction_y: false,

            left_player: (matrix::HEIGHT as u32)/2,
            right_player: (matrix::HEIGHT as u32)/2,

            last_update: 0,
            delay: delay
        }
    }

    pub fn update(&mut self, matrix: &mut matrix::Matrix, now: u32){
        if now - self.last_update > self.delay {

            matrix.set(self.ball_x as usize, self.ball_y as usize, false);

            for i in 0..PLAYER_HEIGHT {
                matrix.set(0, (self.left_player + i) as usize, false);
                matrix.set(matrix::WIDTH - 1, (self.right_player + i) as usize, false);
            }

            self.update_ball();
            self.update_left_player();
            self.update_right_player();

            matrix.set(self.ball_x as usize, self.ball_y as usize, true);

            for i in 0..PLAYER_HEIGHT {
                matrix.set(0, (self.left_player + i) as usize, true);
                matrix.set(matrix::WIDTH - 1, (self.right_player + i) as usize, true);
            }

            self.last_update = now;
        }
    }

    fn update_ball(&mut self){

        if self.ball_y >= matrix::HEIGHT as u32 - 1 {
            self.ball_direction_y = false;
        }

        if self.ball_y <= 0 {
            self.ball_direction_y = true;
        }

        if self.ball_x <= 1 {
            //if self.ball_y >= self.left_player_y && self.ball_y < self.left_player_y + PLAYER_HEIGHT {
            self.ball_direction_x = true;
            //}else{
            //    self.reset();
            //}
        }

        if self.ball_x >= matrix::WIDTH as u32 - 2 {
            //if self.ball_y >= self.right_player_y && self.ball_y < self.right_player_y + PLAYER_HEIGHT {
            self.ball_direction_x = false;
            //}else{
            //    self.reset();
            //}
        }

        match self.ball_direction_x {
            true => self.ball_x += 1,
            false => self.ball_x -= 1
        };

        if self.ball_current_count >= self.ball_counts {
            match self.ball_direction_y {
                true => self.ball_y += 1,
                false => self.ball_y -= 1
            };

            self.ball_current_count = 1;
        }else{
            self.ball_current_count += 1;
        }
    }

    fn update_left_player(&mut self) {

        if self.ball_y > self.left_player + PLAYER_HEIGHT - 1 && self.left_player < matrix::HEIGHT as u32 - PLAYER_HEIGHT {
            self.left_player += 1;
        }

        if self.ball_y < self.left_player && self.left_player > 1 {
            self.left_player -= 1;
        }

    }

    fn update_right_player(&mut self) {

        if self.ball_y > self.right_player + PLAYER_HEIGHT - 1 && self.right_player < matrix::HEIGHT as u32 - PLAYER_HEIGHT {
            self.right_player += 1;
        }

        if self.ball_y < self.right_player && self.right_player > 1 {
            self.right_player -= 1;
        }

    }

    fn reset(&mut self){
        self.ball_x = (matrix::WIDTH as u32)/2 + 1;
        self.ball_y = (matrix::HEIGHT as u32)/2 + 1;

        self.ball_counts = 2;
        self.ball_current_count = 0;

        self.ball_direction_x = true;
        self.ball_direction_y =  false;

        self.left_player = (matrix::HEIGHT as u32)/2;
        self.right_player = (matrix::HEIGHT as u32)/2;
    }

}
