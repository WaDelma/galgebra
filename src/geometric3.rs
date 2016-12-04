//TODO: Implement it.
use alga::general::{AbstractMagma, Field, Additive, Multiplicative};

use std::ops::{Add, Mul, Neg};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Geometric3<F: Clone> {
    b0: F,
    b1: (F, F, F),
    b2: (F, F, F),
    b3: F,
}

impl<F: Clone> Geometric3<F> {
    pub fn dims() -> usize {
        8
    }
}

impl<F: Field> Geometric3<F> {
    pub fn zero() -> Geometric3<F> {
        Geometric3 {
            b0: F::zero(),
            b1: (F::zero(), F::zero(), F::zero()),
            b2: (F::zero(), F::zero(), F::zero()),
            b3: F::zero(),
        }
    }

    pub fn one() -> Geometric3<F> {
        Geometric3 {
            b0: F::one(),
            .. Self::zero()
        }
    }

    pub fn x() -> Geometric3<F> {
        Geometric3 {
            b1: (F::one(), F::zero(), F::zero()),
            ..Self::zero()
        }
    }

    pub fn y() -> Geometric3<F> {
        Geometric3 {
            b1: (F::zero(), F::one(), F::zero()),
            ..Self::zero()
        }
    }

    pub fn z() -> Geometric3<F> {
        Geometric3 {
            b1: (F::zero(), F::zero(), F::one()),
            ..Self::zero()
        }
    }

    pub fn xy() -> Geometric3<F> {
        Geometric3 {
            b2: (F::one(), F::zero(), F::zero()),
            ..Self::zero()
        }
    }

    pub fn yz() -> Geometric3<F> {
        Geometric3 {
            b2: (F::zero(), F::one(), F::zero()),
            ..Self::zero()
        }
    }

    pub fn zx() -> Geometric3<F> {
        Geometric3 {
            b2: (F::zero(), F::zero(), F::one()),
            ..Self::zero()
        }
    }

    pub fn i() -> Geometric3<F> {
        Geometric3 {
            b3: F::one(),
            ..Self::zero()
        }
    }
}

impl<F: Field> Add for Geometric3<F> {
    type Output = Self;
    fn add(mut self, lhs: Self) -> Self::Output {
        self.b0 += lhs.b0;
        self.b1.0 += lhs.b1.0;
        self.b1.1 += lhs.b1.1;
        self.b2.0 += lhs.b2.0;
        self.b2.1 += lhs.b2.1;
        self.b2.2 += lhs.b2.2;
        self.b3 += lhs.b3;
        self
    }
}

impl<F: Field> Neg for Geometric3<F> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.b0 = -self.b0;
        self.b1.0 = -self.b1.0;
        self.b1.1 = -self.b1.1;
        self.b1.2 = -self.b1.2;
        self.b2.0 = -self.b2.0;
        self.b2.1 = -self.b2.1;
        self.b2.2 = -self.b2.2;
        self.b3 = -self.b3;
        self
    }
}

impl<F: Field> AbstractMagma<Additive> for Geometric3<F> {
    fn operate(&self, lhs: &Self) -> Self {
        self.clone() + lhs
    }
}

impl<F: Field> Mul<Geometric3<F>> for Geometric3<F> {
    type Output = Geometric3<F>;
    fn mul(self, lhs: &Self) -> Self::Output {
        unimplemented!();
    }
}

impl<F: Field> AbstractMagma<Multiplicative> for Geometric3<F> {
    fn operate(&self, lhs: &Self) -> Self {
        self.clone() * lhs.clone()
    }
}
