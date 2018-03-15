use ContainerRef;

pub struct ContainerSlice<'a>(::ContainerSlice<'a>);
from_original!(ContainerSlice<'a>);

pub struct Iter<'a> {
    original_slice: &'a ContainerSlice<'a>,
    index: usize,
    index_reversed: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = ContainerRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.index_reversed {
            let i = self.index;
            self.index += 1;
            Some(ContainerRef {
                a: &self.original_slice.0.a[i],
                b: &self.original_slice.0.b[i],
                c: &self.original_slice.0.c[i],
                d: &self.original_slice.0.d[i],
            })
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index_reversed > self.index {
            self.index_reversed -= 1;
            Some(ContainerRef {
                a: &self.original_slice.0.a[self.index_reversed],
                b: &self.original_slice.0.b[self.index_reversed],
                c: &self.original_slice.0.c[self.index_reversed],
                d: &self.original_slice.0.d[self.index_reversed],
            })
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a ContainerSlice<'a> {
    type Item = ContainerRef<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            original_slice: self,
            index: 0,
            index_reversed: self.0.a.len(),
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
