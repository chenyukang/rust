//! Regression test for https://github.com/rust-lang/rust/issues/141742.
//! An unconstrained generic pattern should suggest where to specify its type argument.

struct Wrapper<T> {
    field: T,
}

enum MultipleArgs<'a, T, U> {
    Variant(&'a T, U),
}

fn main() {
    if let Some(whatever) = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    } else {
        todo!();
    }
}

fn unit_variant() {
    if let None = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    }
}

fn struct_pattern() {
    if let Wrapper { field } = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    }
}

fn tuple_struct_with_turbofish() {
    if let Some::<_>(whatever) = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    }
}

fn unit_variant_with_turbofish() {
    if let None::<_> = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    }
}

fn struct_pattern_with_turbofish() {
    if let Wrapper::<_> { field } = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    }
}

fn multiple_args_with_turbofish() {
    if let MultipleArgs::<'_, u8, _>::Variant(_, value) = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    }
}
