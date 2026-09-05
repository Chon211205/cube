use crate::cube::Cube;
use raylib::prelude::*;

pub fn ray_intersect_cube(
    ray: &Ray,
    cube: &Cube,
) -> Option<(f32, Vector3)> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    let mut hit_normal = Vector3::zero();

    if ray.direction.x.abs() > 0.0001 {
        let mut t1 =
            (cube.min.x - ray.position.x)
                / ray.direction.x;

        let mut t2 =
            (cube.max.x - ray.position.x)
                / ray.direction.x;

        let mut normal =
            Vector3::new(-1.0, 0.0, 0.0);

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);

            normal =
                Vector3::new(1.0, 0.0, 0.0);
        }

        if t1 > t_min {
            t_min = t1;
            hit_normal = normal;
        }

        t_max = t_max.min(t2);

        if t_min > t_max {
            return None;
        }
    }

    if ray.direction.y.abs() > 0.0001 {
        let mut t1 =
            (cube.min.y - ray.position.y)
                / ray.direction.y;

        let mut t2 =
            (cube.max.y - ray.position.y)
                / ray.direction.y;

        let mut normal =
            Vector3::new(0.0, -1.0, 0.0);

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);

            normal =
                Vector3::new(0.0, 1.0, 0.0);
        }

        if t1 > t_min {
            t_min = t1;
            hit_normal = normal;
        }

        t_max = t_max.min(t2);

        if t_min > t_max {
            return None;
        }
    }

    if ray.direction.z.abs() > 0.0001 {
        let mut t1 =
            (cube.min.z - ray.position.z)
                / ray.direction.z;

        let mut t2 =
            (cube.max.z - ray.position.z)
                / ray.direction.z;

        let mut normal =
            Vector3::new(0.0, 0.0, -1.0);

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);

            normal =
                Vector3::new(0.0, 0.0, 1.0);
        }

        if t1 > t_min {
            t_min = t1;
            hit_normal = normal;
        }

        t_max = t_max.min(t2);

        if t_min > t_max {
            return None;
        }
    }

    if t_min > 0.001 {
        Some((t_min, hit_normal))
    } else if t_max > 0.001 {
        Some((t_max, hit_normal))
    } else {
        None
    }
}