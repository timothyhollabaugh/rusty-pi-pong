// pong.rs
//
// The logic for pong

use matrix;

const PLAYER_HEIGHT: u32 = 2;

pub struct Pong {
    ball_x: u32,
    ball_y: u32,

    ball_velocity_x: i32,
    ball_velocity_y: i32,

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

            ball_velocity_x: 0,
            ball_velocity_y: 1,

            left_player: (matrix::HEIGHT as u32)/2,
            right_player: (matrix::HEIGHT as u32)/2,

            last_update: 0,
            delay: delay
        }
    }

    pub fn update(&mut self, matrix: &mut matrix::Matrix, now: u32){
        if now - self.last_update > self.delay {

            matrix.set(self.ball_x as usize, self.ball_y as usize, false);

            self.update_ball();

            matrix.set(self.ball_x as usize, self.ball_y as usize, true);

            self.last_update = now;
        }
    }

    fn update_ball(&mut self){

        if self.ball_y >= matrix::HEIGHT as u32 - 1 {
            self.ball_velocity_y = -1 * self.ball_velocity_y.abs();
        }

        if self.ball_y <= 0 {
            self.ball_velocity_y = self.ball_velocity_y.abs();
        }

        if self.ball_x <= 1 {
            //if self.ball_y >= self.left_player_y && self.ball_y < self.left_player_y + PLAYER_HEIGHT {
            self.ball_velocity_x = self.ball_velocity_x.abs();
            //}else{
            //    self.reset();
            //}
        }

        if self.ball_x >= matrix::WIDTH as u32 - 1 {
            //if self.ball_y >= self.right_player_y && self.ball_y < self.right_player_y + PLAYER_HEIGHT {
            self.ball_velocity_x = -self.ball_velocity_x.abs();
            //}else{
            //    self.reset();
            //}
        }

        self.ball_x = (self.ball_x as i32 - self.ball_velocity_x) as u32;
        self.ball_y = (self.ball_y as i32 - self.ball_velocity_y) as u32;

    }

    fn reset(&mut self){
        self.ball_x = (matrix::WIDTH as u32)/2 + 1;
        self.ball_y = (matrix::HEIGHT as u32)/2 + 1;

        self.ball_velocity_x = 1;
        self.ball_velocity_y = 1;

        self.left_player = (matrix::HEIGHT as u32)/2;
        self.right_player = (matrix::HEIGHT as u32)/2;
    }

}
