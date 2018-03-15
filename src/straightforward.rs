use ContainerRef;
use std::slice;

pub struct ContainerSlice<'a>(::ContainerSlice<'a>);
from_original!(ContainerSlice<'a>);

pub struct Iter<'a> {
    a: slice::Iter<'a, u32>,
    b: slice::Iter<'a, f32>,
    c: slice::Iter<'a, f32>,
    d: slice::Iter<'a, f32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = ContainerRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.a.next().map(|a| ContainerRef {
            a,
            b: self.b.next().unwrap(),
            c: self.c.next().unwrap(),
            d: self.d.next().unwrap(),
        })
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.a.next_back().map(|a| ContainerRef {
            a,
            b: self.b.next_back().unwrap(),
            c: self.c.next_back().unwrap(),
            d: self.d.next_back().unwrap(),
        })
    }
}

impl<'a, 'b> IntoIterator for &'b ContainerSlice<'a> {
    type Item = ContainerRef<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            a: self.0.a.iter(),
            b: self.0.b.iter(),
            c: self.0.c.iter(),
            d: self.0.d.iter(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use original::*;
    use test::Bencher;
    use InitFromVectors;

    #[bench]
    fn go_straight(b: &mut Bencher) {
        let slice_obj = ContainerSlice::new(&VEC_A, &VEC_B, &VEC_C, &VEC_D);
        b.iter(|| slice_obj.into_iter().fold(0f32, |acc, x| acc + x.calc()));
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
}
