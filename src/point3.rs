use std::ops;

use crate::vec3::{Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Point3(pub f64, pub f64, pub f64);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        return Point3(x, y, z);
    }
}

impl_op_ex_commutative!(+ |lhs: &Point3, rhs: &Vec3| -> Point3 { Point3(lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2) });
impl_op_ex!(- |lhs: &Point3, rhs: &Vec3| -> Point3 { Point3(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2) });
// impl_op_ex!(+= |lhs: &mut Point3, rhs: &Point3| { lhs.0 += rhs.0; lhs.1 += rhs.1; lhs.2 += rhs.2 });

impl_op_ex!(- |lhs: &Point3, rhs: &Point3| -> Vec3 { Vec3(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2) });

// impl_op_ex!(- |lhs: &Point3, rhs: f64| -> Point3 { Point3(lhs.0 - rhs, lhs.1 - rhs, lhs.2 - rhs) });
// impl_op_ex!(- |lhs: f64, rhs: &Point3| -> Point3 { Point3(lhs - rhs.0, lhs - rhs.1, lhs - rhs.2) });
// impl_op_ex!(-= |lhs: &mut Point3, rhs: &Point3| { lhs.0 -= rhs.0; lhs.1 -= rhs.1; lhs.2 -= rhs.2 });

// impl_op_ex!(- |lhs: &Point3| -> Point3 { Point3(-lhs.0, -lhs.1, -lhs.2) });

// impl_op_ex!(* |lhs: &Point3, rhs: &Point3| -> Point3 { Point3(lhs.0 * rhs.0, lhs.1 * rhs.1, lhs.2 * rhs.2) });
// impl_op_ex_commutative!(* |lhs: &Point3, rhs: f64| -> Point3 { Point3(lhs.0 * rhs, lhs.1 * rhs, lhs.2 * rhs) });
// impl_op_ex!(*= |lhs: &mut Point3, rhs: f64| { lhs.0 *= rhs; lhs.1 *= rhs; lhs.2 *= rhs; });

// impl_op_ex!(/ |lhs: &Point3, rhs: f64| -> Point3 { Point3(lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs) });
// impl_op_ex!(/= |lhs: &mut Point3, rhs: f64| { lhs.0 /= rhs; lhs.1 /= rhs; lhs.2 /= rhs; });