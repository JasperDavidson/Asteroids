mod asteroid;

use ::rand::{thread_rng, Rng};
use asteroid::*;
use nalgebra::*;
use macroquad::{prelude::*, miniquad::start, window};

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut score = 0;

    let starting_x = (screen_width() / 2.0) as f64;
    let starting_y = (screen_height() / 2.0) as f64;
    let displacement = 10.0;
    let mut player = Player{ position: Matrix2x3::new(starting_x, starting_x + displacement, starting_x - displacement,
                                                              starting_y, starting_y + displacement * 2.0, starting_y + displacement * 2.0),
                                    rotation: 2.0, velocity: Vector2::new(2.0, 2.0), angle: 90.0, collision: false };

    let mut bullet_vec: Vec<Bullet> = vec![];
    let bullet_velocity = Vector2::new(5.0, 5.0);
    let bullet_width = 5.0;
    let bullet_height = 10.0;
    let mut fire_time = 0.0;

    let mut asteroid_vec: Vec<Asteroid> = vec![];
    let mut immunity_time = 0.0;

    loop {
        if is_key_pressed(KeyCode::Space) {
            if Bullet::pause_bullet_fire(get_time(), fire_time) {
                bullet_vec.push(player.shoot(bullet_velocity, bullet_width, bullet_height));
                fire_time = get_time();
            }
        }
        if is_key_down(KeyCode::Right) {
            player.move_player("left");
        } 
        if is_key_down(KeyCode::Left) {
            player.move_player("right");
        } 
        if is_key_down(KeyCode::Up) {
            player.move_player("up");
        }

        if player.position.m21 < 1.0 as f64 {
            player.teleport(1.0);
        }
        if player.position.m11 > screen_width() as f64 {
            player.teleport(2.0);
        }
        if player.position.m21 > screen_height() as f64 {
            player.teleport(3.0);
        }
        if player.position.m11 < 1.0 as f64 {
            player.teleport(4.0);
        }

        draw_triangle_lines(Vec2{  x: player.position.m11 as f32, y: player.position.m21 as f32 },
                      Vec2{  x: (player.position.m12) as f32, y: (player.position.m22) as f32 },
                      Vec2{  x: (player.position.m13) as f32, y: (player.position.m23) as f32 }, 2.0, GREEN);

        let mut i = 0;
        while i < bullet_vec.len() {
            let mut remove_bullet = false;
            {
                let bullet = &mut bullet_vec[i];
                bullet.move_bullet();
                if bullet.position.x > screen_width() as f64 || bullet.position.x < 0.0 || bullet.position.y > screen_height() as f64 || bullet.position.y < 0.0 {
                    remove_bullet = true;
                } else {
                    draw_rectangle_lines(bullet.position.x as f32, bullet.position.y as f32, bullet_width as f32, bullet_height as f32, 2.0, PURPLE);
                }
            }
            if remove_bullet || bullet_vec[i].collision {
                bullet_vec.remove(i);
            } else {
                i += 1;
            }
        }

        if asteroid_vec.len() < 6 {
            asteroid_vec.push(Asteroid::spawn(8));
        }

        let mut i = 0;
        while i < asteroid_vec.len() {
            let mut remove_asteroid = false;

            {
                asteroid_vec[i].move_asteroid();

                if asteroid_vec[i].position.y < 1.0 as f64 {
                    asteroid_vec[i].teleport(1.0);
                }
                if asteroid_vec[i].position.x > screen_width() as f64 {
                    asteroid_vec[i].teleport(2.0);
                }
                if asteroid_vec[i].position.y > screen_height() as f64 {
                    asteroid_vec[i].teleport(3.0);
                }
                if asteroid_vec[i].position.x < 1.0 as f64 {
                    asteroid_vec[i].teleport(4.0);
                }

                asteroid_vec[i].collision_check(&mut player, &mut bullet_vec);

                if asteroid_vec[i].collision {
                    remove_asteroid = true;
                    score += 1;
                }

                draw_poly_lines(asteroid_vec[i].position.x as f32, asteroid_vec[i].position.y as f32, asteroid_vec[i].sides, asteroid_vec[i].radius as f32, asteroid_vec[i].rotation as f32, 2.0, GREEN)
            } 
            
            if remove_asteroid {
                if asteroid_vec[i].sides >= 5 {
                    Asteroid::spawn_extra(&mut asteroid_vec);
                }

                asteroid_vec.remove(i);
            } else {
                i += 1;
            }
        }

        draw_text(&format!("{}", score), 100.0, 100.0, 50.0, GREEN);

        if player.collision {
            break
        }

        next_frame().await;
    }
}
