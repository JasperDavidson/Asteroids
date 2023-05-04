use std::thread::spawn;

use ::rand::{Rng, thread_rng};
use nalgebra::*;
use macroquad::prelude::*;

pub struct Player {
    pub position: Matrix2x3<f64>,
    pub rotation: f64,
    pub velocity: Vector2<f64>,
    pub angle: f64,
    pub collision: bool,
}

impl Player {
    pub fn center(&self) -> Vector2<f64> {
        Vector2::new((self.position.m11 + self.position.m12 + self.position.m13) / 3.0,
         (self.position.m21 + self.position.m22 + self.position.m23) / 3.0)
    }

    pub fn move_player(&mut self, direction: &str) {
        if direction == "left" {
            self.rotate(false);
        } else if direction == "right" {
            self.rotate(true);
        } 
    
        if direction == "up" {
            let sin = (-self.angle).to_radians().sin();
            let cos = (-self.angle).to_radians().cos();

            self.position.m11 += self.velocity.x * cos;
            self.position.m12 += self.velocity.x * cos;
            self.position.m13 += self.velocity.x * cos;
            self.position.m21 += self.velocity.y * sin;
            self.position.m22 += self.velocity.y * sin;
            self.position.m23 += self.velocity.y * sin;
        }
    }

    pub fn shoot(&mut self, velocity: Vector2<f64>, width: f64, height: f64) -> Bullet {
        let sin = (-self.angle).to_radians().sin();
        let cos = (-self.angle).to_radians().cos();

        Bullet { position: Vector2::new(self.position.m11, self.position.m21), velocity: velocity, width: width, height: height, sin: sin, cos: cos, collision: false }
    }

    pub fn rotate(&mut self, opposite: bool) {
        let center = ((self.position.m11 + self.position.m12 + self.position.m13) / 3.0,
                                  (self.position.m21 + self.position.m22 + self.position.m23) / 3.0);
        let mut position = Matrix2x3::new(self.position.m11 - center.0, self.position.m12 - center.0, self.position.m13 - center.0,
                                                                          self.position.m21 - center.1, self.position.m22 - center.1, self.position.m23 - center.1);
        if !opposite {
            self.angle -= self.rotation;
            let rotation_matrix = Matrix2::new(self.rotation.to_radians().cos() as f64, -self.rotation.to_radians().sin() as f64,
                                                                               self.rotation.to_radians().sin() as f64, self.rotation.to_radians().cos() as f64);
            position = rotation_matrix * position;
        } else {
            self.angle += self.rotation;
            let rotation_matrix = Matrix2::new(self.rotation.to_radians().cos() as f64, self.rotation.to_radians().sin() as f64,
                                                                   -self.rotation.to_radians().sin() as f64, self.rotation.to_radians().cos() as f64);
            position = rotation_matrix * position;
        }
        
        self.position = Matrix2x3::new(position.m11 + center.0, position.m12 + center.0, position.m13 + center.0,
                                       position.m21 + center.1, position.m22 + center.1, position.m23 + center.1);
    }

    pub fn teleport(&mut self, side: f64) {
        if side == 1.0 {
            self.position.m21 += screen_height() as f64;
            self.position.m22 += screen_height() as f64;
            self.position.m23 += screen_height() as f64;
        } else if side == 2.0 {
            self.position.m11 -= screen_width() as f64;
            self.position.m12 -= screen_width() as f64;
            self.position.m13 -= screen_width() as f64;
        } else if side == 3.0 {
            self.position.m21 -= screen_height() as f64;
            self.position.m22 -= screen_height() as f64;
            self.position.m23 -= screen_height() as f64;
        } else if side == 4.0 {
            self.position.m11 += screen_width() as f64;
            self.position.m12 += screen_width() as f64;
            self.position.m13 += screen_width() as f64;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bullet {
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub width: f64,
    pub height: f64,
    pub sin: f64,
    pub cos: f64,
    pub collision: bool,
}

impl Bullet {
    pub fn move_bullet(&mut self) {
        self.position.x += self.velocity.x * self.cos;
        self.position.y += self.velocity.y * self.sin;
    }

    pub fn pause_bullet_fire(time: f64, fire_time: f64) -> bool {
        if time - fire_time >= 0.3 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Asteroid {
    pub position: Vector2<f64>,
    pub rotation: f64,
    pub velocity: Vector2<f64>,
    pub sides: u8,
    pub radius: f64,
    pub collision: bool,
}

impl Asteroid {
    pub fn spawn(sides: u8) -> Asteroid {
        let mut rng = thread_rng();
        let side = rng.gen_range(1..=4);
        let mut x = 0.0;
        let mut y = 0.0;

        let asteroid_rotation = rng.gen_range(0.0..=360.0);
        let asteroid_velocity = Vector2::new(rng.gen_range(-1.5..=1.5), rng.gen_range(-1.5..=1.5));
        let asteroid_radius = 75.0;
        let asteroid_sides = sides;

        if side == 1 {
            x = rng.gen_range(0..=screen_width() as usize) as f64;
        } else if side == 2 {
            x = screen_width() as f64;
            y = rng.gen_range(0..=screen_height() as usize) as f64;
        } else if side == 3 {
            y = screen_height() as f64;
            x = rng.gen_range(0..=screen_width() as usize) as f64;
        } else {
            y = rng.gen_range(0..=screen_height() as usize) as f64;
        }

        Asteroid { position: Vector2::new(x, y), rotation: asteroid_rotation, velocity: asteroid_velocity, sides: asteroid_sides, radius: asteroid_radius, collision: false }
    }

    pub fn move_asteroid(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn teleport(&mut self, side: f64) {
        if side == 1.0 {
            self.position.y += screen_height() as f64;
        } else if side == 2.0 {
            self.position.x -= screen_width() as f64;
        } else if side == 3.0 {
            self.position.y -= screen_height() as f64;
        } else if side == 4.0 {
            self.position.x += screen_width() as f64;
        }
    }

    pub fn collision_check(&mut self, player: &mut Player, bullet_vec: &mut Vec<Bullet>) {
        for mut bullet in bullet_vec {
            if self.position.x - 55.0 <= bullet.position.x && bullet.position.x <= self.position.x + 55.0
            && self.position.y - 55.0 <= bullet.position.y && bullet.position.y <= self.position.y + 55.0 {
                self.collision = true;
                bullet.collision = true;
            }
        }

        if self.position.x - 60.0 <= player.center().x && player.center().x <= self.position.x + 60.0
        && self.position.y - 60.0 <= player.center().y && player.center().y <= self.position.y + 60.0  {
            player.collision = true;
        }
    }

    pub fn spawn_extra(asteroid_vec: &mut Vec<Asteroid>) {
        for i in 0..asteroid_vec.len() {
            if asteroid_vec[i].collision {
                let new_side_amount = asteroid_vec[i].sides - 1;

                let mut asteroid_one = Asteroid::spawn(new_side_amount);
                asteroid_one.position = asteroid_vec[i].position - Vector2::new(40.0, 40.0);
                asteroid_one.velocity = asteroid_vec[i].velocity;
                asteroid_one.velocity.x *= -1.0;
                asteroid_one.velocity.y *= -1.0;

                let mut asteroid_two = Asteroid::spawn(new_side_amount);
                asteroid_two.position = asteroid_vec[i].position + Vector2::new(40.0, 40.0);
                asteroid_two.velocity = asteroid_vec[i].velocity;

                asteroid_vec.push(asteroid_one);
                asteroid_vec.push(asteroid_two);
            }
        }
    }
}
