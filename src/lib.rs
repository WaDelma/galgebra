//#![recursion_limit="128"]
extern crate num_traits as num;
#[macro_use]
extern crate alga;
#[macro_use]
extern crate alga_derive;
extern crate typenum;
extern crate generic_array;

use alga::general::Operator;

pub mod generic;

pub mod geometric2;
// pub mod geometric3;

//TODO: Figure out if these are useful and where to use them :D
#[derive(Clone, Copy)]
pub struct Inner;
impl Operator for Inner {
    fn operator_token() -> Self {
        Inner
    }
}

#[derive(Clone, Copy)]
pub struct Outer;
impl Operator for Outer {
    fn operator_token() -> Self {
        Outer
    }
}
