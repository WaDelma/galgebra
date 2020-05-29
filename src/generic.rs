use alga::general::TwoSidedInverse;
use typenum::{Pow, Unsigned, U2};

use generic_array::{ArrayLength, GenericArray};

use num::Zero;

use alga::general::{AbstractMagma, AbstractModule, Additive, Identity};

use std::ops::{Add, Index, Mul, Neg};

// TODO: This should derive AbelianGroup, but for some reason it fails.
#[derive(Alga)]
#[alga_traits(
    Semigroup(Additive),
    // GroupAbelian(Additive),
    Where = "
        N: Unsigned,
        U2: Pow<N>,
        <U2 as Pow<N>>::Output: ArrayLength<f64>"
)]
pub struct Multivector<N>(GenericArray<f64, <U2 as Pow<N>>::Output>)
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>;

// TODO: This should work, but it just breaks.
// impl<N> Multivector<N>
// where
//     N: Unsigned,
//     U2: Pow<N>,
//     <U2 as Pow<N>>::Output: ArrayLength<f64>,
// {
//     fn from_slice(data: &[f64]) -> Multivector<N> {
//         Multivector::<N>(GenericArray::<f64, <U2 as Pow<N>>::Output>::from_slice(
//             data,
//         ))
//     }
// }

pub fn from_slice<N>(data: &[f64]) -> Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    Multivector::<N>(GenericArray::<f64, <U2 as Pow<N>>::Output>::from_slice(
        data,
    ))
}

impl<N> Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    pub fn dims() -> usize {
        <U2 as Pow<N>>::Output::to_usize()
    }
}

// #[test]
// fn dims_works() {
//     use typenum::U2;
//     assert_eq!(4, Multivector::<U2>::default().dims());
// }

impl<N> Clone for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn clone(&self) -> Self {
        Multivector::<N>(self.0.clone())
    }
}

impl<N> PartialEq for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn eq(&self, lhs: &Self) -> bool {
        self.0.iter().zip(lhs.0.iter()).all(|(a, b)| a == b)
    }
}

impl<N> Default for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
    GenericArray<f64, <U2 as Pow<N>>::Output>: Default,
{
    fn default() -> Self {
        Multivector::<N>(<GenericArray<f64, <U2 as Pow<N>>::Output> as Default>::default())
    }
}

#[derive(Default)]
struct Empty;

// #[test]
// fn default_works() {
//     use typenum::U10;
//     // GenericArray::<u8, U1>::default();
//     let m = Multivector::<U10>::default();
//     for v in m {
//         assert_eq!(0, v);
//     }
// }

impl<N> Zero for Multivector<N>
where
    N: Unsigned,
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

impl<N> Identity<Additive> for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
    GenericArray<f64, <U2 as Pow<N>>::Output>: Default,
{
    fn identity() -> Self {
        Self::default()
    }
}

impl<N> Add for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    type Output = Self;
    fn add(mut self, lhs: Self) -> Self::Output {
        for (us, them) in self.0.iter_mut().zip(lhs.0.iter()) {
            *us += *them;
        }
        self
    }
}

impl<N> Neg for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for v in self.0.iter_mut() {
            *v = -*v;
        }
        self
    }
}

impl<N> TwoSidedInverse<Additive> for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn two_sided_inverse(&self) -> Self {
        -self.clone()
    }
}

impl<N> AbstractMagma<Additive> for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    fn operate(&self, lhs: &Self) -> Self {
        self.clone() + lhs.clone()
    }
}

impl<N> Mul<f64> for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    type Output = Multivector<N>;
    fn mul(mut self, lhs: f64) -> Self::Output {
        for v in self.0.iter_mut() {
            *v *= lhs;
        }
        self
    }
}

// impl<N> AbstractModule for Multivector<N>
// where
//     N: Unsigned,
//     U2: Pow<N>,
//     <U2 as Pow<N>>::Output: ArrayLength<f64>,
// {
//     type AbstractRing = f64;

//     fn multiply_by(&self, r: Self::AbstractRing) -> Self {
//         self.clone() * r
//     }
// }

impl<N> Mul<Multivector<N>> for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
    <U2 as Pow<N>>::Output: ArrayLength<f64>,
{
    type Output = Multivector<N>;
    fn mul(self, lhs: Self) -> Self::Output {
        unimplemented!();
        // let (a0, b0) = (|| self.b0.clone(), || lhs.b0.clone());
        // let (a1, b1) = (|| self.b1.0.clone(), || lhs.b1.0.clone());
        // let (a2, b2) = (|| self.b1.1.clone(), || lhs.b1.1.clone());
        // let (a3, b3) = (|| self.b2.clone(), || lhs.b2.clone());
        // let p0 = a0()*b0() + a1()*b1() + a2()*b2() - a3()*b3();
        // let p1 = a0()*b1() + a1()*b0() + a3()*b2() - a2()*b3();
        // let p2 = a0()*b2() + a3()*b0() + a1()*b3() - a3()*b1();
        // let p3 = a0()*b3() + a3()*b0() + a1()*b2() - a2()*b1();
        // Geometric2 {
        //     b0: p0,
        //     b1: (p1, p2),
        //     b2: p3,
        // }
    }
}

impl<N> Index<usize> for Multivector<N>
where
    N: Unsigned,
    U2: Pow<N>,
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
    // let t = Multivector::<U5>::from_slice(&t[..]);
    let t = from_slice::<U5>(&t[..]);
    assert_eq!([0.], t[0]);
    assert_eq!([1., 2., 3., 4., 5.], t[1]);
    assert_eq!([6., 7., 8., 9., 10., 11., 12., 13., 14., 15.], t[2]);
    assert_eq!([16., 17., 18., 19., 20., 21., 22., 23., 24., 25.], t[3]);
    assert_eq!([26., 27., 28., 29., 30.], t[4]);
    assert_eq!([31.], t[5]);
}

fn pascal_row(row: usize) -> PascalRow {
    PascalRow {
        row,
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
    assert_eq!(vec![/*    */ 1,], pascal_row(0).collect::<Vec<_>>());
    assert_eq!(vec![/*   */ 1, 1,], pascal_row(1).collect::<Vec<_>>());
    assert_eq!(vec![/*  */ 1, 2, 1,], pascal_row(2).collect::<Vec<_>>());
    assert_eq!(vec![/* */ 1, 3, 3, 1,], pascal_row(3).collect::<Vec<_>>());
    assert_eq!(vec![/**/ 1, 4, 6, 4, 1,], pascal_row(4).collect::<Vec<_>>());
}
