mod framebuffer;
mod sphere;
mod ray_intersect;

use framebuffer::Framebuffer;
use sphere::Sphere;
use ray_intersect::ray_intersect_sphere;
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
        .title("Ray Tracing - Phong Lighting")
        .build();

    rl.set_target_fps(60);



    let background = Color::new(20, 20, 30, 255);



    let sphere = Sphere {
        center: Vector3::new(0.0, 0.0, 0.0),
        radius: 1.5,

        // COLOR BASE
        color: Color::new(200, 50, 50, 255),
    };



    let light_position = Vector3::new(
        -3.0,
        4.0,
        2.0,
    );

    let light_color = Color::WHITE;


    let ambient_strength = 0.15;
    let diffuse_strength = 0.75;
    let specular_strength = 0.8;


    let shininess = 64.0;



    let mut yaw = 0.0_f32;
    let mut pitch = 0.0_f32;
    let mut camera_distance = 5.0_f32;

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

        camera_distance =
            camera_distance.clamp(2.5, 12.0);

        pitch =
            pitch.clamp(-1.4, 1.4);



        let camera_position = Vector3::new(
            sphere.center.x
                + camera_distance
                    * yaw.sin()
                    * pitch.cos(),

            sphere.center.y
                + camera_distance
                    * pitch.sin(),

            sphere.center.z
                + camera_distance
                    * yaw.cos()
                    * pitch.cos(),
        );


        let forward =
            (sphere.center - camera_position)
                .normalize();

        let world_up =
            Vector3::new(0.0, 1.0, 0.0);

        let right =
            forward
                .cross(world_up)
                .normalize();

        let up =
            right
                .cross(forward)
                .normalize();



        let mut framebuffer =
            Framebuffer::new(width, height);

        framebuffer.set_background_color(background);
        framebuffer.clear();



        for y in 0..height {
            for x in 0..width {

                let px =
                    (
                        2.0
                            * (
                                (x as f32 + 0.5)
                                    / width as f32
                            )
                            - 1.0
                    )
                        * aspect_ratio
                        * scale;

                let py =
                    (
                        1.0
                            - 2.0
                                * (
                                    (y as f32 + 0.5)
                                        / height as f32
                                )
                    )
                        * scale;


                let direction =
                    (
                        forward
                            + right * px
                            + up * py
                    )
                        .normalize();

                let ray = Ray {
                    position: camera_position,
                    direction,
                };



                if let Some(t) =
                    ray_intersect_sphere(
                        &ray,
                        &sphere,
                    )
                {

                    let hit_point =
                        ray.position
                            + ray.direction * t;



                    let normal =
                        (
                            hit_point
                                - sphere.center
                        )
                            .normalize();



                    let light_direction =
                        (
                            light_position
                                - hit_point
                        )
                            .normalize();


                    let view_direction =
                        (
                            camera_position
                                - hit_point
                        )
                            .normalize();



                    let base_r =
                        sphere.color.r as f32;

                    let base_g =
                        sphere.color.g as f32;

                    let base_b =
                        sphere.color.b as f32;


                    let ambient_r =
                        base_r * ambient_strength;

                    let ambient_g =
                        base_g * ambient_strength;

                    let ambient_b =
                        base_b * ambient_strength;


                    let diffuse_factor =
                        normal
                            .dot(light_direction)
                            .max(0.0);

                    let diffuse_r =
                        base_r
                            * diffuse_factor
                            * diffuse_strength;

                    let diffuse_g =
                        base_g
                            * diffuse_factor
                            * diffuse_strength;

                    let diffuse_b =
                        base_b
                            * diffuse_factor
                            * diffuse_strength;


                    let incoming_light =
                        -light_direction;


                    let reflected =
                        reflect(
                            incoming_light,
                            normal,
                        )
                            .normalize();

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



                    let final_r =
                        ambient_r
                            + diffuse_r
                            + specular_r;

                    let final_g =
                        ambient_g
                            + diffuse_g
                            + specular_g;

                    let final_b =
                        ambient_b
                            + diffuse_b
                            + specular_b;

                    let final_color =
                        Color::new(
                            clamp_color(final_r),
                            clamp_color(final_g),
                            clamp_color(final_b),
                            255,
                        );

                    framebuffer
                        .set_pixel_color(
                            x,
                            y,
                            final_color,
                        );
                } else {
                    framebuffer
                        .set_pixel_color(
                            x,
                            y,
                            background,
                        );
                }
            }
        }



        let texture =
            rl.load_texture_from_image(
                &thread,
                framebuffer.get_image(),
            )
                .expect(
                    "No se pudo crear la textura"
                );


        let mut d =
            rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_texture(
            &texture,
            0,
            0,
            Color::WHITE,
        );

        d.draw_text(
            "Ray Tracing - Phong Lighting",
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
            "Flechas : rotar arriba / abajo",
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