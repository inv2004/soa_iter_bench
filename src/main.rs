#![feature(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate soa_derive;
extern crate rand;
extern crate test;

use rand::{thread_rng, Rng};
use test::Bencher;
use std::iter;

#[derive(StructOfArray)]
#[soa_derive = "Debug, PartialEq"]
pub struct S {
    a: u32,
    b: f32,
    c: f32,
    d: f32,
}
impl<'a> SRef<'a> {
    fn calc(&self) -> f32 {
        *self.a as f32 + self.b + self.c + self.d
    }
}

mod old_slice {
    use std::slice;
    use super::SRef;

    pub struct SSlice<'a> {
        pub a: &'a [u32],
        pub b: &'a [f32],
        pub c: &'a [f32],
        pub d: &'a [f32],
    }

    pub struct Iter<'a> {
        pub a: slice::Iter<'a, u32>,
        pub b: slice::Iter<'a, f32>,
        pub c: slice::Iter<'a, f32>,
        pub d: slice::Iter<'a, f32>,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = SRef<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            let a = self.a.next();
            let b = self.b.next();
            let c = self.c.next();
            let d = self.d.next();

            if a.is_none() {
                None
            } else {
                Some(SRef {
                    a: a.unwrap(),
                    b: b.unwrap(),
                    c: c.unwrap(),
                    d: d.unwrap(),
                })
            }
        }
    }

    impl<'a, 'b> IntoIterator for &'b SSlice<'a> {
        type Item = SRef<'a>;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            Iter {
                a: self.a.iter(),
                b: self.b.iter(),
                c: self.c.iter(),
                d: self.d.iter(),
            }
        }
    }
}

mod old2_slice {
    use std::slice;
    use super::SRef;

    pub struct SSlice<'a> {
        pub a: &'a [u32],
        pub b: &'a [f32],
        pub c: &'a [f32],
        pub d: &'a [f32],
    }

    pub struct Iter<'a> {
        pub a: slice::Iter<'a, u32>,
        pub b: slice::Iter<'a, f32>,
        pub c: slice::Iter<'a, f32>,
        pub d: slice::Iter<'a, f32>,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = SRef<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            self.a.next().and_then(|a| {
                Some(SRef {
                    a,
                    b: &self.b.next().unwrap(),
                    c: &self.c.next().unwrap(),
                    d: &self.d.next().unwrap(),
                })
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.a.size_hint()
        }
    }

    impl<'a, 'b> IntoIterator for &'b SSlice<'a> {
        type Item = SRef<'a>;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            Iter {
                a: self.a.iter(),
                b: self.b.iter(),
                c: self.c.iter(),
                d: self.d.iter(),
            }
        }
    }
}

mod new_slice {
    use std::slice;
    use std::iter;
    use super::SRef;

    pub struct SSlice<'a> {
        pub a: &'a [u32],
        pub b: &'a [f32],
        pub c: &'a [f32],
        pub d: &'a [f32],
    }

    pub struct Iter<'a>(
        iter::Zip<
            iter::Zip<iter::Zip<slice::Iter<'a, u32>, slice::Iter<'a, f32>>, slice::Iter<'a, f32>>,
            slice::Iter<'a, f32>,
        >,
    );

    impl<'a> Iterator for Iter<'a> {
        type Item = SRef<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0
                .next()
                .and_then(|(((a, b), c), d)| Some(SRef { a, b, c, d }))
        }
    }

    impl<'a> DoubleEndedIterator for Iter<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0
                .next_back()
                .and_then(|(((a, b), c), d)| Some(SRef { a, b, c, d }))
        }
    }

    impl<'a> IntoIterator for &'a SSlice<'a> {
        type Item = SRef<'a>;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            Iter(
                self.a
                    .iter()
                    .zip(self.b.iter())
                    .zip(self.c.iter())
                    .zip(self.d.iter()),
            )
        }
    }
}

mod new2_slice {
    use super::SRef;

    pub struct SSlice<'a> {
        pub a: &'a [u32],
        pub b: &'a [f32],
        pub c: &'a [f32],
        pub d: &'a [f32],
    }

    pub struct Iter<'a> {
        sl: &'a SSlice<'a>,
        i: usize,
        i_rev: usize,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = SRef<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i < self.i_rev {
                let i = self.i;
                self.i += 1;
                Some(SRef {
                    a: &self.sl.a[i],
                    b: &self.sl.b[i],
                    c: &self.sl.c[i],
                    d: &self.sl.d[i],
                })
            } else {
                None
            }
        }
    }

    impl<'a> DoubleEndedIterator for Iter<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.i_rev > self.i {
                self.i_rev -= 1;
                Some(SRef {
                    a: &self.sl.a[self.i_rev],
                    b: &self.sl.b[self.i_rev],
                    c: &self.sl.c[self.i_rev],
                    d: &self.sl.d[self.i_rev],
                })
            } else {
                None
            }
        }
    }

    impl<'a> IntoIterator for &'a SSlice<'a> {
        type Item = SRef<'a>;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            Iter {
                sl: &self,
                i: 0,
                i_rev: self.a.len(),
            }
        }
    }

}

lazy_static! {
    static ref VEC_A: Vec<u32> = {
        iter::repeat(())
            .map(|()| thread_rng().gen())
            .take(100_000)
            .collect()
    };
    static ref VEC_B: Vec<f32> = {
        iter::repeat(())
            .map(|()| thread_rng().gen())
            .take(100_000)
            .collect()
    };
    static ref VEC_C: Vec<f32> = {
        iter::repeat(())
            .map(|()| thread_rng().gen())
            .take(100_000)
            .collect()
    };
    static ref VEC_D: Vec<f32> = {
        iter::repeat(())
            .map(|()| thread_rng().gen())
            .take(100_000)
            .collect()
    };
}

fn main() {
    let sl_old = old_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    let sl_new = new_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    let sl_new2 = new2_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };

    let mut acc = 0.0;
    for (a, b, c, d) in izip!(sl_old.a, sl_old.b, sl_old.c, sl_old.d) {
        let r = SRef { a, b, c, d };
        acc += r.calc();
    }
    println!("{}", acc);

    let mut acc = 0.0;
    for (((a, b), c), d) in sl_old
        .a
        .iter()
        .zip(sl_old.b.iter())
        .zip(sl_old.c.iter())
        .zip(sl_old.d.iter())
    {
        let r = SRef { a, b, c, d };
        acc += r.calc();
    }
    println!("{}", acc);

    let mut acc = 0.0;
    for r in &sl_old {
        acc += r.calc();
    }
    println!("{}", acc);

    let mut acc = 0.0;
    for r in &sl_new {
        acc += r.calc();
    }
    println!("{}", acc);

    let mut acc = 0.0;
    for r in &sl_new2 {
        acc += r.calc();
    }
    println!("{}", acc);
}

#[bench]
fn test_izip(b: &mut Bencher) {
    let sl_old = old_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for (a, b, c, d) in izip!(sl_old.a, sl_old.b, sl_old.c, sl_old.d) {
            let r = SRef { a, b, c, d };
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_zip(b: &mut Bencher) {
    let sl_old = old_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for (((a, b), c), d) in sl_old
            .a
            .iter()
            .zip(sl_old.b.iter())
            .zip(sl_old.c.iter())
            .zip(sl_old.d.iter())
        {
            let r = SRef { a, b, c, d };
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_zip_without_ref(b: &mut Bencher) {
    let sl_old = old_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for (((a, b), c), d) in sl_old
            .a
            .iter()
            .zip(sl_old.b.iter())
            .zip(sl_old.c.iter())
            .zip(sl_old.d.iter())
        {
            acc += *a as f32 + b + c + d
        }
        acc
    });
}

#[bench]
fn test_zip_rev(b: &mut Bencher) {
    let sl_old = old_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for (((a, b), c), d) in sl_old
            .a
            .iter()
            .zip(sl_old.b.iter())
            .zip(sl_old.c.iter())
            .zip(sl_old.d.iter())
            .rev()
        {
            let r = SRef { a, b, c, d };
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_old(b: &mut Bencher) {
    let sl_old = old_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in &sl_old {
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_old2(b: &mut Bencher) {
    let sl_old = old2_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in &sl_old {
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_new(b: &mut Bencher) {
    let sl_new = new_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in &sl_new {
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_new2(b: &mut Bencher) {
    let sl_new2 = new2_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in &sl_new2 {
            acc += r.calc();
        }
        acc
    });
}

#[test]
fn test_new2_rev_test() {
    let sl_new2 = new2_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    let v1 = sl_new2
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    let v2 = sl_new2.into_iter().rev().collect::<Vec<_>>();
    assert_eq!(v1, v2);
}

#[bench]
fn test_new2_rev(b: &mut Bencher) {
    let sl_new2 = new2_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in sl_new2.into_iter().rev() {
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_iter_opt(b: &mut Bencher) {
    let sl_soa = SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in &sl_soa {
            acc += r.calc();
        }
        acc
    });
}

#[bench]
fn test_new_rev(b: &mut Bencher) {
    let sl_new = new_slice::SSlice {
        a: &VEC_A,
        b: &VEC_B,
        c: &VEC_C,
        d: &VEC_D,
    };
    b.iter(|| {
        let mut acc = 0.0;
        for r in sl_new.into_iter().rev() {
            acc += r.calc();
        }
        acc
    });
}
