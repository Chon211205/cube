mod framebuffer;
mod cube;
mod ray_intersect;

use framebuffer::Framebuffer;
use cube::Cube;
use ray_intersect::ray_intersect_cube;
use raylib::prelude::*;

fn clamp_color(value: f32) -> u8 {
    value.clamp(0.0, 255.0) as u8
}

fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
    vector - normal * 2.0 * vector.dot(normal)
}

fn main() {
    let width = 800;
    let height = 600;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Ray Tracing - Cubo")
        .build();

    rl.set_target_fps(60);

    let background = Color::new(20, 20, 30, 255);

    let cube = Cube {
        min: Vector3::new(-1.5, -1.5, -1.5),
        max: Vector3::new(1.5, 1.5, 1.5),
        color: Color::new(50, 120, 220, 255),
    };

    let cube_center = (cube.min + cube.max) * 0.5;

    let light_position = Vector3::new(-4.0, 5.0, 4.0);
    let light_color = Color::WHITE;

    let ambient_strength = 0.15;
    let diffuse_strength = 0.75;
    let specular_strength = 0.8;
    let shininess = 64.0;

    let mut yaw = 0.5_f32;
    let mut pitch = 0.3_f32;
    let mut camera_distance = 7.0_f32;

    let fov = 60.0_f32.to_radians();
    let aspect_ratio = width as f32 / height as f32;
    let scale = (fov / 2.0).tan();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let rotation_speed = 1.5 * dt;
        let zoom_speed = 2.0 * dt;

        if rl.is_key_down(KeyboardKey::KEY_A) {
            yaw -= rotation_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            yaw += rotation_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            pitch += rotation_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            pitch -= rotation_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera_distance -= zoom_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera_distance += zoom_speed;
        }

        camera_distance = camera_distance.clamp(4.0, 15.0);
        pitch = pitch.clamp(-1.4, 1.4);

        let camera_position = Vector3::new(
            cube_center.x + camera_distance * yaw.sin() * pitch.cos(),
            cube_center.y + camera_distance * pitch.sin(),
            cube_center.z + camera_distance * yaw.cos() * pitch.cos(),
        );

        let forward = (cube_center - camera_position).normalize();

        let world_up = Vector3::new(0.0, 1.0, 0.0);

        let right = forward.cross(world_up).normalize();
        let up = right.cross(forward).normalize();

        let mut framebuffer = Framebuffer::new(width, height);

        framebuffer.set_background_color(background);
        framebuffer.clear();

        for y in 0..height {
            for x in 0..width {
                let px =
                    (2.0 * ((x as f32 + 0.5) / width as f32) - 1.0)
                        * aspect_ratio
                        * scale;

                let py =
                    (1.0 - 2.0 * ((y as f32 + 0.5) / height as f32))
                        * scale;

                let direction =
                    (forward + right * px + up * py).normalize();

                let ray = Ray {
                    position: camera_position,
                    direction,
                };

                if let Some((t, normal)) =
                    ray_intersect_cube(&ray, &cube)
                {
                    let hit_point =
                        ray.position + ray.direction * t;

                    let light_direction =
                        (light_position - hit_point).normalize();

                    let view_direction =
                        (camera_position - hit_point).normalize();

                    let base_r = cube.color.r as f32;
                    let base_g = cube.color.g as f32;
                    let base_b = cube.color.b as f32;

                    let ambient_r = base_r * ambient_strength;
                    let ambient_g = base_g * ambient_strength;
                    let ambient_b = base_b * ambient_strength;

                    let diffuse_factor =
                        normal.dot(light_direction).max(0.0);

                    let diffuse_r =
                        base_r * diffuse_factor * diffuse_strength;

                    let diffuse_g =
                        base_g * diffuse_factor * diffuse_strength;

                    let diffuse_b =
                        base_b * diffuse_factor * diffuse_strength;

                    let incoming_light = -light_direction;

                    let reflected =
                        reflect(incoming_light, normal).normalize();

                    let specular_factor =
                        view_direction
                            .dot(reflected)
                            .max(0.0)
                            .powf(shininess);

                    let specular_r =
                        light_color.r as f32
                            * specular_factor
                            * specular_strength;

                    let specular_g =
                        light_color.g as f32
                            * specular_factor
                            * specular_strength;

                    let specular_b =
                        light_color.b as f32
                            * specular_factor
                            * specular_strength;

                    let final_color = Color::new(
                        clamp_color(
                            ambient_r
                                + diffuse_r
                                + specular_r
                        ),
                        clamp_color(
                            ambient_g
                                + diffuse_g
                                + specular_g
                        ),
                        clamp_color(
                            ambient_b
                                + diffuse_b
                                + specular_b
                        ),
                        255,
                    );

                    framebuffer.set_pixel_color(
                        x,
                        y,
                        final_color,
                    );
                } else {
                    framebuffer.set_pixel_color(
                        x,
                        y,
                        background,
                    );
                }
            }
        }

        let texture = rl
            .load_texture_from_image(
                &thread,
                framebuffer.get_image(),
            )
            .expect("No se pudo crear la textura");

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_texture(
            &texture,
            0,
            0,
            Color::WHITE,
        );

        d.draw_text(
            "Ray Tracing - Cubo",
            20,
            20,
            24,
            Color::WHITE,
        );

        d.draw_text(
            "A / D : rotar alrededor",
            20,
            55,
            18,
            Color::WHITE,
        );

        d.draw_text(
            "Flechas : arriba / abajo",
            20,
            80,
            18,
            Color::WHITE,
        );

        d.draw_text(
            "W / S : acercar / alejar",
            20,
            105,
            18,
            Color::WHITE,
        );
    }
}