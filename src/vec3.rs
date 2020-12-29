use std::ops;
use serde::{Serialize, Deserialize};
use rand::{Rng, RngCore};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3([f64; 3]);

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]) });
impl_op_ex_commutative!(+ |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs[0] + rhs, lhs[1] + rhs, lhs[2] + rhs) });
impl_op_ex!(+= |lhs: &mut Vec3, rhs: &Vec3| { lhs[0] += rhs[0]; lhs[1] += rhs[1]; lhs[2] += rhs[2] });

impl_op_ex!(- |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]) });
impl_op_ex!(- |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs[0] - rhs, lhs[1] - rhs, lhs[2] - rhs) });
impl_op_ex!(- |lhs: f64, rhs: &Vec3| -> Vec3 { Vec3::new(lhs - rhs[0], lhs - rhs[1], lhs - rhs[2]) });
impl_op_ex!(-= |lhs: &mut Vec3, rhs: &Vec3| { lhs[0] -= rhs[0]; lhs[1] -= rhs[1]; lhs[2] -= rhs[2] });

impl_op_ex!(- |lhs: &Vec3| -> Vec3 { Vec3::new(-lhs[0], -lhs[1], -lhs[2]) });

impl_op_ex!(* |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs[0] * rhs[0], lhs[1] * rhs[1], lhs[2] * rhs[2]) });
impl_op_ex_commutative!(* |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs[0] * rhs, lhs[1] * rhs, lhs[2] * rhs) });
impl_op_ex!(*= |lhs: &mut Vec3, rhs: f64| { lhs[0] *= rhs; lhs[1] *= rhs; lhs[2] *= rhs; });

impl_op_ex!(/ |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs[0] / rhs, lhs[1] / rhs, lhs[2] / rhs) });
impl_op_ex!(/= |lhs: &mut Vec3, rhs: f64| { lhs[0] /= rhs; lhs[1] /= rhs; lhs[2] /= rhs; });

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3([x, y, z]);
    }

    pub fn length(&self) -> f64 {
        return self.length2().sqrt();
    }

    pub fn length2(&self) -> f64 {
        return self.dot(*self);
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        return self[0] * other[0] + self[1] * other[1] + self[2] * other[2];
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        return Vec3::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        );
    }

    pub fn normalize(&self) -> Vec3 {
        return *self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        return self[0].abs() < eps && self[1].abs() < eps && self[2].abs() < eps;
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        return *self - 2.0 * n * self.dot(n);
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv.dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_par = -(1.0 - r_out_perp.length2()).abs().sqrt() * n;
        return r_out_perp + r_out_par;
    }

    pub fn random(rng: &mut dyn RngCore) -> Vec3 {
        return Vec3::new(
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
        );
    }

    pub fn random_in_unit_sphere(rng: &mut dyn RngCore) -> Vec3 {
        loop {
            let v = Vec3::random(rng);
            if v.length2() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk(rng: &mut dyn RngCore) -> Vec3 {
        let v = Vec3::random_in_unit_sphere(rng);
        return Vec3::new(
            v[0],
            v[1],
            0.0,
        );
    }

    pub fn random_unit_vector(rng: &mut dyn RngCore) -> Vec3 {
        return Vec3::random_in_unit_sphere(rng).normalize();
    }
}