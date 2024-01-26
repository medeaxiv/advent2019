use nalgebra::{vector, Vector2, Vector3};

pub type Vec2 = Vector2<i64>;

pub fn vec2(x: i64, y: i64) -> Vec2 {
    vector![x, y]
}

pub type Vec3 = Vector3<i64>;

pub fn vec3(x: i64, y: i64, z: i64) -> Vec3 {
    vector![x, y, z]
}
