use typenum::{U2, Unsigned, Pow};

use generic_array::{GenericArray, ArrayLength};

use num::Zero;

use alga::general::{AbstractMagma, Additive};

use std::ops::{Index, Add};

#[derive(Alga)]
#[alga_traits(Semigroup(Additive), Where = "U2: Pow<N>, <U2 as Pow<N>>::Output: ArrayLength<f64>")]
pub struct Multivector<N: Unsigned>(GenericArray<f64, <U2 as Pow<N>>::Output>)
    where U2: Pow<N>,
          <U2 as Pow<N>>::Output: ArrayLength<f64>;

// TODO: This should work, but it just breaks.
// impl<N: Unsigned> Multivector<N>
//     where U2: Pow<N>,
//           <U2 as Pow<N>>::Output: ArrayLength<f64>,
// {
//     fn from_slice(data: &[f64]) -> Multivector<N> {
//         Multivector::<N>(GenericArray::<f64, <U2 as Pow<N>>::Output>::from_slice(data))
//     }
// }

pub fn from_slice<N: Unsigned>(data: &[f64]) -> Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    Multivector::<N>(GenericArray::<f64, <U2 as Pow<N>>::Output>::from_slice(data))
}

impl<N: Unsigned> Clone for Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn clone(&self) -> Self {
        Multivector::<N>(self.0.clone())
    }
}

impl<N: Unsigned> PartialEq for Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn eq(&self, lhs: &Self) -> bool {
        self.0.iter()
            .zip(lhs.0.iter())
            .all(|(a, b)| a == b)
    }
}

impl<N: Unsigned> Default for Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
    GenericArray<f64, <U2 as Pow<N>>::Output>: Default,
{
    fn default() -> Self {
        Multivector::<N>(<GenericArray<f64, <U2 as Pow<N>>::Output> as Default>::default())
    }
}

impl<N: Unsigned> Zero for Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
    GenericArray<f64, <U2 as Pow<N>>::Output>: Default,
{
    fn zero() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl<N: Unsigned> Add for Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    type Output = Self;
    fn add(mut self, lhs: Self) -> Self::Output {
        for (us, them) in self.0.iter_mut()
                .zip(lhs.0.iter()) {
            *us += *them;
        }
        self
    }
}

impl<N: Unsigned> AbstractMagma<Additive> for Multivector<N>
  where
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn operate(&self, lhs: &Self) -> Self {
        self.clone() + lhs.clone()
    }
}

impl<N: Unsigned> Index<usize> for Multivector<N>
    where U2: Pow<N>,
          <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    type Output = [f64];
    fn index(&self, n: usize) -> &Self::Output {
        assert!(n < 2usize.pow(N::to_usize() as u32));
        let n = n + 1;
        let (index, size) = pascal_row(N::to_usize())
            .take(n)
            .fold((0, 0), |(s, ln), n| (s + ln, n));
        &self.0[index..(index + size)]
    }
}

#[test]
fn indexing_works() {
    use typenum::U5;
    let t: Vec<f64> = (0u64..).take(2usize.pow(5u32)).map(|n| n as f64).collect();
    let t = from_slice::<U5>(&t[..]);
    assert_eq!([0.0], t[0]);
    assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0], t[1]);
    assert_eq!([6.0, 7.0, 8.0, 9.0, 10., 11., 12., 13., 14., 15.], t[2]);
    assert_eq!([16., 17., 18., 19., 20., 21., 22., 23., 24., 25.], t[3]);
    assert_eq!([26., 27., 28., 29., 30.], t[4]);
    assert_eq!([31.], t[5]);
}

fn pascal_row(row: usize) -> PascalRow {
    PascalRow {
        row: row,
        col: 1,
        value: 1,
    }
}

struct PascalRow {
    row: usize,
    col: usize,
    value: usize,
}

impl Iterator for PascalRow {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col <= self.row + 1 {
            let cur = self.value;
            self.value = (self.value * (self.row + 1 - self.col)) / self.col;
            self.col += 1;
            Some(cur)
        } else {
            None
        }
    }
}

#[test]
fn pascal_row_works() {
    assert_eq!(vec![        1,        ], pascal_row(0).collect::<Vec<_>>());
    assert_eq!(vec![      1,  1,      ], pascal_row(1).collect::<Vec<_>>());
    assert_eq!(vec![    1,  2,  1,    ], pascal_row(2).collect::<Vec<_>>());
    assert_eq!(vec![  1,  3,  3,  1,  ], pascal_row(3).collect::<Vec<_>>());
    assert_eq!(vec![1,  4,  6,  4,  1,], pascal_row(4).collect::<Vec<_>>());
}
