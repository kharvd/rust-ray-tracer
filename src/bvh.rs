use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;

use rand::{Rng, RngCore};

use crate::bounding_box::BBox;
use crate::geometry::{HitRecord, Hittable};
use crate::ray::Ray;

pub enum BVHNode<T: Hittable> {
    Internal {
        bbox: BBox,
        left: Box<BVHNode<T>>,
        right: Box<BVHNode<T>>,
    },
    Leaf {
        hittable: T,
    },
}

impl<T: Hittable + Clone> BVHNode<T> {
    pub fn from_shapes(rng: &mut dyn RngCore, shapes: &mut [T]) -> BVHNode<T> {
        if shapes.len() == 1 {
            return BVHNode::Leaf { hittable: shapes[0].clone() };
        }

        let axis = rng.gen_range(0..3);
        shapes.sort_by(|s1, s2| BVHNode::compare(axis, s1, s2));

        let (left_slice, right_slice) = shapes.split_at_mut(shapes.len() / 2);

        let left = BVHNode::from_shapes(rng, left_slice);
        let right = BVHNode::from_shapes(rng, right_slice);
        let bbox = BBox::surrounding_box(
            left.bounding_box().unwrap(),
            right.bounding_box().unwrap(),
        );

        BVHNode::Internal { bbox, left: Box::new(left), right: Box::new(right) }
    }

    fn compare(axis: usize, shape1: &T, shape2: &T) -> Ordering {
        let min1 = shape1.bounding_box().unwrap().min[axis];
        let min2 = shape2.bounding_box().unwrap().min[axis];
        min1.partial_cmp(&min2).unwrap()
    }
}

impl<T: Hittable + Debug> Debug for BVHNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BVHNode::Internal { bbox, left, right } => {
                f.debug_struct("BVHNode")
                    .field("bbox", bbox)
                    .field("left", left)
                    .field("right", right)
                    .finish()
            }
            BVHNode::Leaf { hittable: shape } => shape.fmt(f)
        }
    }
}

impl<T: Hittable> Hittable for BVHNode<T> {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            BVHNode::Leaf { hittable } => hittable.hit_by(ray, t_min, t_max),

            BVHNode::Internal { bbox, left, right } => {
                if !bbox.hit(ray, t_min, t_max) {
                    None
                } else {
                    let hit_left = left.hit_by(ray, t_min, t_max);
                    let t_max1 = if let Some(ref rec) = hit_left { rec.t } else { t_max };
                    let hit_right = right.hit_by(ray, t_min, t_max1);
                    hit_right.or(hit_left)
                }
            }
        }
    }

    fn bounding_box(&self) -> Option<BBox> {
        match self {
            BVHNode::Internal { bbox, .. } => Some(*bbox),
            BVHNode::Leaf { hittable: shape } => shape.bounding_box()
        }
    }
}
