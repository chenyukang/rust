// The ICE only occurs when using both `associated_const_equality`
// and `type_alias_impl_trait` together.
#![feature(associated_const_equality)]
#![feature(type_alias_impl_trait)]

trait Trait<T> {
    const K: T;
}

type Foo<'a> = impl Trait<&'a str, K = { () }>;
//~^ ERROR: unconstrained opaque type

fn main() {}
