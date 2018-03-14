#![feature(test)]
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate itertools;
extern crate rand;
extern crate test;

use rand::{thread_rng, Rng};
use test::Bencher;
use std::iter;

#[derive(Debug)]
pub struct SRef<'a> {
    a: &'a u32,
    b: &'a f32,
    c: &'a f32,
    d: &'a f32,
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

  pub struct Iter<'a>{
      pub a: slice::Iter<'a, u32>,
      pub b: slice::Iter<'a, f32>,
      pub c: slice::Iter<'a, f32>,
      pub d: slice::Iter<'a, f32>,
  }

  impl<'a> Iterator for Iter<'a> {
    type Item = SRef<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.a.size_hint()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a.next();
        let b = self.b.next();
        let c = self.c.next();
        let d = self.d.next();
        
        if a.is_none() {
            None
        } else {
            Some(SRef{
                a:a.unwrap(),
                b:b.unwrap(),
                c:c.unwrap(),
                d:d.unwrap(),
            })
        }
    }
  }

  impl<'a,'b> IntoIterator for &'b SSlice<'a> {
    type Item = SRef<'a>;
    type IntoIter = Iter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        Iter{
          a:self.a.iter(),
          b:self.b.iter(),
          c:self.c.iter(),
          d:self.d.iter(),
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

  pub struct Iter<'a>(iter::Zip<iter::Zip<iter::Zip<slice::Iter<'a,u32>, slice::Iter<'a,f32>>, slice::Iter<'a,f32>>, slice::Iter<'a,f32>>);

  impl<'a> Iterator for Iter<'a> {
    type Item = SRef<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((((a,b),c),d)) = self.0.next() {
            Some(SRef{a, b, c, d})
        } else {
            None
        }
    }
  }

  impl<'a> IntoIterator for &'a SSlice<'a> {
    type Item = SRef<'a>;
    type IntoIter = Iter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.a.iter().zip(self.b.iter()).zip(self.c.iter()).zip(self.d.iter()))
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

  pub struct Iter<'a>{
      sl: &'a SSlice<'a>,
      i: usize,
      len: usize,
  }

  impl<'a> Iterator for Iter<'a> {
    type Item = SRef<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.len {
            let r = SRef{
              a:&self.sl.a[self.i],
              b:&self.sl.b[self.i],
              c:&self.sl.b[self.i],
              d:&self.sl.b[self.i],
            };
            self.i += 1;
            Some(r)
        } else {
            None
        }
    }
  }

  impl<'a> IntoIterator for &'a SSlice<'a> {
    type Item = SRef<'a>;
    type IntoIter = Iter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        Iter{
            sl:&self,
            i:0,
            len: self.a.len(),
        }
    }
  }
}

lazy_static! {
    static ref VEC_A: Vec<u32> = { iter::repeat(()).map(|()| thread_rng().gen()).take(100_000).collect() };
    static ref VEC_B: Vec<f32> = { iter::repeat(()).map(|()| thread_rng().gen()).take(100_000).collect() };
    static ref VEC_C: Vec<f32> = { iter::repeat(()).map(|()| thread_rng().gen()).take(100_000).collect() };
    static ref VEC_D: Vec<f32> = { iter::repeat(()).map(|()| thread_rng().gen()).take(100_000).collect() };
}

fn main() {
    let sl_old = old_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
    let sl_new = new_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
    let sl_new2 = new2_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};

    let mut acc = 0.0;
    for (a,b,c,d) in izip!(sl_old.a, sl_old.b, sl_old.c, sl_old.d) {
        let r = SRef{a,b,c,d};
        acc += r.calc();
    }
    println!("{}", acc);

    let mut acc = 0.0;
    for (((a,b),c),d) in sl_old.a.iter().zip(sl_old.b.iter()).zip(sl_old.c.iter()).zip(sl_old.d.iter()) {
        let r = SRef{a,b,c,d};
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
  let sl_old = old_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
  b.iter(|| {
    let mut acc = 0.0;
    for (a,b,c,d) in izip!(sl_old.a, sl_old.b, sl_old.c, sl_old.d) {
        let r = SRef{a,b,c,d};
        acc += r.calc();
    }
    acc
  });
}

#[bench]
fn test_zip(b: &mut Bencher) {
  let sl_old = old_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
  b.iter(|| {
    let mut acc = 0.0;
    for (((a,b),c),d) in sl_old.a.iter().zip(sl_old.b.iter()).zip(sl_old.c.iter()).zip(sl_old.d.iter()) {
        let r = SRef{a,b,c,d};
        acc += r.calc();
    }
    acc
  });
}

#[bench]
fn test_old(b: &mut Bencher) {
  let sl_old = old_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
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
  let sl_new = new_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
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
  let sl_new2 = new2_slice::SSlice{a:&VEC_A, b:&VEC_B, c:&VEC_C, d:&VEC_D};
  b.iter(|| {
    let mut acc = 0.0;
    for r in &sl_new2 {
        acc += r.calc();
    }
    acc
  });
}




