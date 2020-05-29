use alga::general::{
    AbstractMagma, AbstractModule, Additive, Field, Identity, Multiplicative, TwoSidedInverse,
};
use num::{One, Zero};

use std::ops::{Add, Mul, Neg};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Alga)]
#[alga_traits(GroupAbelian(Additive), Where = "F: Field")]
pub struct Geometric2<F: Field> {
    b0: F,
    b1: (F, F),
    b2: F,
}

impl<F: Field> Geometric2<F> {
    pub fn dims() -> usize {
        4
    }
}

impl<F: Field> Geometric2<F> {
    pub fn zero() -> Geometric2<F> {
        Geometric2 {
            b0: F::zero(),
            b1: (F::zero(), F::zero()),
            b2: F::zero(),
        }
    }

    pub fn one() -> Geometric2<F> {
        Geometric2 {
            b0: F::one(),
            ..Self::zero()
        }
    }

    pub fn x() -> Geometric2<F> {
        Geometric2 {
            b1: (F::one(), F::zero()),
            ..Self::zero()
        }
    }

    pub fn y() -> Geometric2<F> {
        Geometric2 {
            b1: (F::zero(), F::one()),
            ..Self::zero()
        }
    }

    pub fn i() -> Geometric2<F> {
        Geometric2 {
            b2: F::one(),
            ..Self::zero()
        }
    }
}

impl<F: Field> Add for Geometric2<F> {
    type Output = Self;
    fn add(mut self, lhs: Self) -> Self::Output {
        self.b0 += lhs.b0;
        self.b1.0 += lhs.b1.0;
        self.b1.1 += lhs.b1.1;
        self.b2 += lhs.b2;
        self
    }
}

impl<F: Field> Zero for Geometric2<F> {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_zero(&self) -> bool {
        self.clone() == Self::zero()
    }
}

impl<F: Field> Identity<Additive> for Geometric2<F> {
    fn identity() -> Self {
        Self::zero()
    }
}

impl<F: Field> Neg for Geometric2<F> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.b0 = -self.b0;
        self.b1.0 = -self.b1.0;
        self.b1.1 = -self.b1.1;
        self.b2 = -self.b2;
        self
    }
}

impl<F: Field> TwoSidedInverse<Additive> for Geometric2<F> {
    fn two_sided_inverse(&self) -> Self {
        -self.clone()
    }
}

impl<F: Field> AbstractMagma<Additive> for Geometric2<F> {
    fn operate(&self, lhs: &Self) -> Self {
        self.clone() + lhs.clone()
    }
}

// impl_abelian!(<Additive> for Geometric2<F> where F: Field);

impl<F: Field> Mul<F> for Geometric2<F> {
    type Output = Geometric2<F>;
    fn mul(self, lhs: F) -> Self::Output {
        let mul = || lhs.clone();
        Geometric2 {
            b0: mul() * self.b0,
            b1: (mul() * self.b1.0, mul() * self.b1.1),
            b2: mul() * self.b2,
        }
    }
}

impl<F: Field> AbstractModule for Geometric2<F> {
    type AbstractRing = F;

    fn multiply_by(&self, r: Self::AbstractRing) -> Self {
        self.clone() * r
    }
}

impl<F: Field> Mul<Geometric2<F>> for Geometric2<F> {
    type Output = Geometric2<F>;
    fn mul(self, lhs: Self) -> Self::Output {
        let (a0, b0) = (|| self.b0.clone(), || lhs.b0.clone());
        let (a1, b1) = (|| self.b1.0.clone(), || lhs.b1.0.clone());
        let (a2, b2) = (|| self.b1.1.clone(), || lhs.b1.1.clone());
        let (a3, b3) = (|| self.b2.clone(), || lhs.b2.clone());
        let p0 = a0() * b0() + a1() * b1() + a2() * b2() - a3() * b3();
        let p1 = a0() * b1() + a1() * b0() + a3() * b2() - a2() * b3();
        let p2 = a0() * b2() + a3() * b0() + a1() * b3() - a3() * b1();
        let p3 = a0() * b3() + a3() * b0() + a1() * b2() - a2() * b1();
        Geometric2 {
            b0: p0,
            b1: (p1, p2),
            b2: p3,
        }
    }
}

impl<F: Field> One for Geometric2<F> {
    fn one() -> Self {
        Self::one()
    }
}

impl<F: Field> Identity<Multiplicative> for Geometric2<F> {
    fn identity() -> Self {
        Self::one()
    }
}

impl<F: Field> AbstractMagma<Multiplicative> for Geometric2<F> {
    fn operate(&self, lhs: &Self) -> Self {
        self.clone() * lhs.clone()
    }
}

#[cfg(test)]
mod test {
    use super::Geometric2 as G2;
    use alga::general::{Additive, Identity, Multiplicative};

    #[test]
    fn product_parallel_commutative() {
        let a = G2::<f64>::x();
        let b = G2::x() * 2.;
        assert_eq!(a * b, b * a);
    }

    #[test]
    fn squaring_works() {
        let a = G2::<f64>::x();
        assert_eq!(G2::one(), a * a);
        let b = G2::<f64>::y();
        assert_eq!(G2::one(), b * b);
        let i = G2::<f64>::i();
        assert_eq!(-G2::one(), i * i);
    }

    #[test]
    fn product_perpendicular_anticommutative() {
        let a = G2::<f64>::x();
        let b = G2::y();
        assert_eq!(a * b, -b * a);
    }

    #[test]
    fn identities_work() {
        let a = G2::<f64>::one() + G2::x() + G2::y() + G2::i();
        let zero = Identity::<Additive>::identity();
        let one: G2<_> = Identity::<Multiplicative>::identity();
        assert_eq!(a, a + zero);
        assert_eq!(a, zero + a);
        assert_eq!(a, a * one);
        assert_eq!(a, one * a);
    }

    //TODO: Quickcheck
}
