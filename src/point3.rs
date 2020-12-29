use std::ops;

use serde::{Serialize, Deserialize};
use crate::vec3::{Vec3};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point3([f64; 3]);

impl Point3 {
    pub fn zero() -> Point3 {
        Point3::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        return Point3([x, y, z]);
    }
}

impl Index<usize> for Point3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Point3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl_op_ex!(+ |lhs: &Point3, rhs: &Vec3| -> Point3 { Point3::new(lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]) });
impl_op_ex!(- |lhs: &Point3, rhs: &Vec3| -> Point3 { Point3::new(lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]) });
impl_op_ex!(- |lhs: &Point3, rhs: &Point3| -> Vec3 { Vec3::new(lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]) });