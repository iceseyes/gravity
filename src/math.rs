use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;

pub fn align(target: Vec3, spacing: f64) -> Vec3 {
    target.map(|c| (c / spacing).floor() * spacing)
}

pub fn edges_from_point(center: Vec3, half_size: i32, spacing: f64) -> (Vec3, Vec3) {
    let v = f64::from(half_size) * spacing;
    let offset = Vec3::new(v, v, v);
    let min = center - offset;
    let max = center + offset;

    (min, max)
}

pub fn origin(center: Vec3, min: Vec3, max: Vec3) -> Vec3 {
    let pick = |c, min, max| if min <= 0.0 && max >= 0.0 { 0.0 } else { c };

    Vec3::new(
        pick(center.x, min.x, max.x),
        pick(center.y, min.y, max.y),
        pick(center.z, min.z, max.z),
    )
}
