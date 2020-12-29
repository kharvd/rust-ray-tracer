use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;

use rand::{Rng, thread_rng};

use crate::bounding_box::BBox;
use crate::geometry::{HitRecord, Hittable, Shape};
use crate::ray::Ray;

pub enum BVHNode {
    Internal {
        bbox: BBox,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
    Leaf {
        shape: Shape,
    },
}

impl BVHNode {
    pub fn from_shapes(shapes: &mut [Shape]) -> BVHNode {
        if shapes.len() == 1 {
            return BVHNode::Leaf { shape: shapes[0].clone() };
        }

        let axis = thread_rng().gen_range(0..3);
        shapes.sort_by(|s1, s2| BVHNode::compare(axis, s1, s2));

        let (left_slice, right_slice) = shapes.split_at_mut(shapes.len() / 2);

        let left = BVHNode::from_shapes(left_slice);
        let right = BVHNode::from_shapes(right_slice);
        let bbox = BBox::surrounding_box(
            left.bounding_box().unwrap(),
            right.bounding_box().unwrap(),
        );

        BVHNode::Internal { bbox, left: Box::new(left), right: Box::new(right) }
    }

    fn compare(axis: usize, shape1: &Shape, shape2: &Shape) -> Ordering {
        let min1 = shape1.bounding_box().unwrap().min.as_slice()[axis];
        let min2 = shape2.bounding_box().unwrap().min.as_slice()[axis];
        min1.partial_cmp(&min2).unwrap()
    }
}

impl Debug for BVHNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BVHNode::Internal { bbox, left, right } => {
                f.debug_struct("BVHNode")
                    .field("bbox", bbox)
                    .field("left", left)
                    .field("right", right)
                    .finish()
            }
            BVHNode::Leaf { shape } => shape.fmt(f)
        }
    }
}

impl Hittable for BVHNode {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            BVHNode::Leaf { shape } => shape.hit_by(ray, t_min, t_max),

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
            BVHNode::Leaf { shape } => shape.bounding_box()
        }
    }
}
