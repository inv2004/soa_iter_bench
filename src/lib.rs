#![feature(test)]
#[macro_use]
extern crate soa_derive;
extern crate rand;
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate itertools;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[derive(StructOfArray)]
#[soa_derive = "Debug, PartialEq"]
pub struct Container {
    a: u32,
    b: f32,
    c: f32,
    d: f32,
}

impl<'a> ContainerRef<'a> {
    pub fn calc(&self) -> f32 {
        *self.a as f32 + self.b + self.c + self.d
    }
}

pub trait InitFromVectors<'a> {
    fn new(a: &'a [u32], b: &'a [f32], c: &'a [f32], d: &'a [f32]) -> Self;
}

impl<'a, T: From<ContainerSlice<'a>>> InitFromVectors<'a> for T {
    fn new(a: &'a [u32], b: &'a [f32], c: &'a [f32], d: &'a [f32]) -> Self {
        ContainerSlice { a, b, c, d }.into()
    }
}

macro_rules! from_original {
    ($t: ty) => {
        impl<'a> From<$crate::ContainerSlice<'a>> for $t {
            fn from(original: $crate::ContainerSlice<'a>) -> Self {
                Self { 0: original }
            }
        }
    };
}

pub mod straightforward;
pub mod zipped;
pub mod indexed;

#[cfg(test)]
mod original {
    use rand::{thread_rng, Rng};
    pub use test::Bencher;
    use super::*;

    lazy_static! {
        pub static ref VEC_A: Vec<u32> = { (0..100_000).map(|_| thread_rng().gen()).collect() };
        pub static ref VEC_B: Vec<f32> = { (0..100_000).map(|_| thread_rng().gen()).collect() };
        pub static ref VEC_C: Vec<f32> = { (0..100_000).map(|_| thread_rng().gen()).collect() };
        pub static ref VEC_D: Vec<f32> = { (0..100_000).map(|_| thread_rng().gen()).collect() };
    }

    #[bench]
    fn go_straight(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        let slice_obj = &slice_obj;
        b.iter(|| slice_obj.into_iter().fold(0f32, |acc, x| acc + x.calc()));
    }

    #[bench]
    fn go_backwards(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        let slice_obj = &slice_obj;
        b.iter(|| {
            slice_obj
                .into_iter()
                .rev()
                .fold(0f32, |acc, x| acc + x.calc())
        });
    }

    #[bench]
    fn straight_izip(b: &mut Bencher) {
        let slice_object = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| {
            izip!(
                slice_object.a,
                slice_object.b,
                slice_object.c,
                slice_object.d
            ).fold(0f32, |acc, (a, b, c, d)| {
                acc + ContainerRef { a, b, c, d }.calc()
            })
        });
    }

    #[bench]
    fn straight_zip(b: &mut Bencher) {
        let slice_object = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| {
            slice_object
                .a
                .iter()
                .zip(slice_object.b.iter())
                .zip(slice_object.c.iter())
                .zip(slice_object.d.iter())
                .fold(0f32, |acc, (((a, b), c), d)| {
                    acc + ContainerRef { a, b, c, d }.calc()
                })
        });
    }
}
