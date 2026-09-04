use raylib::prelude::*;
use crate::sphere::Sphere;

pub fn ray_intersect_sphere(
    ray: &Ray,
    sphere: &Sphere,
) -> Option<f32> {
    let oc = ray.position - sphere.center;

    let a = ray.direction.dot(ray.direction);

    let b =
        2.0 * oc.dot(ray.direction);

    let c =
        oc.dot(oc)
        - sphere.radius * sphere.radius;

    let discriminant =
        b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_discriminant =
        discriminant.sqrt();

    let t1 =
        (-b - sqrt_discriminant)
        / (2.0 * a);

    let t2 =
        (-b + sqrt_discriminant)
        / (2.0 * a);

    if t1 > 0.001 {
        Some(t1)
    } else if t2 > 0.001 {
        Some(t2)
    } else {
        None
    }
}