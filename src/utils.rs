use crate::V3;
use cgmath::{InnerSpace, Vector2};
use rand::rngs::ThreadRng;
use rand::Rng;

pub fn random_vector(rng: &mut ThreadRng) -> V3 {
    let distr = rand::distributions::Uniform::new(-1.0, 1.0);
    V3::new(rng.sample(distr), rng.sample(distr), rng.sample(distr)).normalize()
}

pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: V3) -> V3 {
    let v = random_vector(rng);
    if v.dot(normal) > 0.0 {
        v
    } else {
        -v
    }
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> V3 {
    let distrib = rand::distributions::Uniform::new(-1.0, 1.0);
    let p2 = Vector2::new(rng.sample(distrib), rng.sample(distrib));
    p2.normalize().extend(0.0)
}