mod framebuffer;
mod cube;
mod ray_intersect;

use cube::Cube;
use framebuffer::Framebuffer;
use ray_intersect::ray_intersect_cube;
use raylib::prelude::*;

struct TextureData {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl TextureData {
    fn load(path: &str) -> Self {
        let img = image::open(path)
            .expect("No se pudo cargar la textura")
            .to_rgba8();

        let width = img.width();
        let height = img.height();
        let pixels = img.into_raw();

        Self {
            width,
            height,
            pixels,
        }
    }

    fn get_color(&self, x: u32, y: u32) -> Color {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        let index = ((y * self.width + x) * 4) as usize;

        Color::new(
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
            self.pixels[index + 3],
        )
    }
}

fn get_texture_color(
    texture: &TextureData,
    hit_point: Vector3,
    normal: Vector3,
    cube: &Cube,
) -> Color {
    let size_x = cube.max.x - cube.min.x;
    let size_y = cube.max.y - cube.min.y;
    let size_z = cube.max.z - cube.min.z;

    let (mut u, mut v);

    if normal.x.abs() > 0.5 {
        u = (hit_point.z - cube.min.z) / size_z;
        v = (hit_point.y - cube.min.y) / size_y;

        if normal.x > 0.0 {
            u = 1.0 - u;
        }
    } else if normal.y.abs() > 0.5 {
        u = (hit_point.x - cube.min.x) / size_x;
        v = (hit_point.z - cube.min.z) / size_z;

        if normal.y < 0.0 {
            v = 1.0 - v;
        }
    } else {
        u = (hit_point.x - cube.min.x) / size_x;
        v = (hit_point.y - cube.min.y) / size_y;

        if normal.z < 0.0 {
            u = 1.0 - u;
        }
    }

    u = u.clamp(0.0, 1.0);
    v = v.clamp(0.0, 1.0);

    v = 1.0 - v;

    let tx =
        (u * (texture.width - 1) as f32) as u32;

    let ty =
        (v * (texture.height - 1) as f32) as u32;

    texture.get_color(tx, ty)
}

fn main() {
    let width = 800;
    let height = 600;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Ray Tracing - Cubo con textura")
        .build();

    rl.set_target_fps(60);

    let background =
        Color::new(20, 20, 30, 255);

    let cube = Cube {
        min: Vector3::new(-1.5, -1.5, -1.5),
        max: Vector3::new(1.5, 1.5, 1.5),
    };

    let cube_center =
        (cube.min + cube.max) * 0.5;

    let texture_path =
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/marmol.jpg"
        );

    let texture =
        TextureData::load(texture_path);

    let mut yaw = 0.5_f32;
    let mut pitch = 0.3_f32;
    let mut camera_distance = 7.0_f32;

    let fov =
        60.0_f32.to_radians();

    let aspect_ratio =
        width as f32 / height as f32;

    let scale =
        (fov / 2.0).tan();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let rotation_speed =
            1.5 * dt;

        let zoom_speed =
            2.0 * dt;

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
            camera_distance.clamp(4.0, 15.0);

        pitch =
            pitch.clamp(-1.4, 1.4);

        let camera_position =
            Vector3::new(
                cube_center.x
                    + camera_distance
                        * yaw.sin()
                        * pitch.cos(),

                cube_center.y
                    + camera_distance
                        * pitch.sin(),

                cube_center.z
                    + camera_distance
                        * yaw.cos()
                        * pitch.cos(),
            );

        let forward =
            (cube_center - camera_position)
                .normalize();

        let world_up =
            Vector3::new(
                0.0,
                1.0,
                0.0,
            );

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

        framebuffer
            .set_background_color(background);

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

                if let Some((t, normal)) =
                    ray_intersect_cube(
                        &ray,
                        &cube,
                    )
                {
                    let hit_point =
                        ray.position
                            + ray.direction * t;

                    let color =
                        get_texture_color(
                            &texture,
                            hit_point,
                            normal,
                            &cube,
                        );

                    framebuffer
                        .set_pixel_color(
                            x,
                            y,
                            color,
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

        let screen_texture =
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
            &screen_texture,
            0,
            0,
            Color::WHITE,
        );

        d.draw_text(
            "Cubo con textura",
            20,
            20,
            24,
            Color::WHITE,
        );

        d.draw_text(
            "A / D : rotar",
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
            "W / S : zoom",
            20,
            105,
            18,
            Color::WHITE,
        );
    }
}