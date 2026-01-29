//@ edition: 2024
//! Regression test for ICE #148124
//! Tests that the compiler doesn't panic with index out of bounds
//! when encountering associated const bindings with complex generic arguments
//! from opaque types.

#![feature(min_generic_const_args)]
//~^ WARN the feature `min_generic_const_args` is incomplete
#![feature(impl_trait_in_assoc_type)]

trait Trait<T> {
    const K: T;
}
trait Discard {
    type Out;
}

trait Inner {
    type T;
}

impl<'a> Inner for &'a i32 {
    type T = impl Trait<<fn(&'a str) -> &'a str as Discard>::Out, K = { () }>;
    //~^ ERROR the type of the associated constant `K` must not reference generic parameters from outer items
    //~| ERROR expected `{type error}`, found const tuple
    //~| ERROR use of trait associated const without `#[type_const]`
    //~| ERROR unconstrained opaque type
    //~| ERROR the type of the associated constant `K` must not reference generic parameters from outer items
    //~| ERROR expected `{type error}`, found const tuple
    //~| ERROR use of trait associated const without `#[type_const]`
}

fn main() {}
