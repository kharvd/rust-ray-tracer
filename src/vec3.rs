use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use rand::random;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point = Vec3;

pub fn point(x: f64, y: f64, z: f64) -> Point {
    return Vec3(x, y, z);
}

pub type Color = Vec3;

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3(-self.0, -self.1, -self.2);
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return rhs * self;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        );
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        return self * (1.0 / rhs);
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        return self.length2().sqrt();
    }

    pub fn length2(&self) -> f64 {
        return self.dot(self);
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        return Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        );
    }

    pub fn normalize(&self) -> Vec3 {
        return *self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        return self.0.abs() < eps && self.1.abs() < eps && self.2.abs() < eps;
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        return *self - 2.0 * (*n) * self.dot(n);
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv.dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * ((*uv) + cos_theta * (*n));
        let r_out_par = -(1.0 - r_out_perp.length2()).abs().sqrt() * (*n);
        return r_out_perp + r_out_par;
    }

    pub fn random() -> Vec3 {
        return Vec3(
            2.0 * random::<f64>() - 1.0,
            2.0 * random::<f64>() - 1.0,
            2.0 * random::<f64>() - 1.0,
        );
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random();
            if v.length2() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let v = Vec3::random_in_unit_sphere();
        return Vec3(
            v.0,
            v.1,
            0.0,
        );
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::random_in_unit_sphere().normalize();
    }
}

