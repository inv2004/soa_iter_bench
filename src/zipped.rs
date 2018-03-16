use ContainerRef;
use std::{iter, slice};

pub struct ContainerSlice<'a>(::ContainerSlice<'a>);
from_original!(ContainerSlice<'a>);

pub struct Iter<'a>(
    iter::Zip<
        iter::Zip<iter::Zip<slice::Iter<'a, u32>, slice::Iter<'a, f32>>, slice::Iter<'a, f32>>,
        slice::Iter<'a, f32>,
    >,
);

impl<'a> Iterator for Iter<'a> {
    type Item = ContainerRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .and_then(|(((a, b), c), d)| Some(ContainerRef { a, b, c, d }))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .and_then(|(((a, b), c), d)| Some(ContainerRef { a, b, c, d }))
    }
}

impl<'a> IntoIterator for &'a ContainerSlice<'a> {
    type Item = ContainerRef<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(
            self.0
                .a
                .iter()
                .zip(self.0.b.iter())
                .zip(self.0.c.iter())
                .zip(self.0.d.iter()),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use original::*;
    use test::Bencher;
    use InitFromVectors;

    #[bench]
    fn go_straight_fold(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| slice_obj.into_iter().fold(0f32, |acc, x| acc + x.calc()));
    }

    #[bench]
    fn go_straight_for(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| {
            let mut acc = 0f32;
            for x in slice_obj.into_iter() {
                acc += x.calc();
            }
            acc
        });
    }

    #[bench]
    fn go_backwards(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| {
            slice_obj
                .into_iter()
                .rev()
                .fold(0f32, |acc, x| acc + x.calc())
        });
    }

    #[bench]
    fn go_backwards_for(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| {
            let mut acc = 0f32;
            for x in slice_obj.into_iter().rev() {
                acc += x.calc();
            }
            acc
        });
    }
}
