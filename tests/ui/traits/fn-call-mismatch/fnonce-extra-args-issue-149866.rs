// ICE: `debug_assert_eq!(params_with_generics.len(), matched_inputs.len())`
// issue: rust-lang/rust#149866
//
// This test verifies that providing incorrect number of arguments to FnOnce
// callable produces a proper error instead of an ICE.

pub fn ice<U, F>(f: F, u: U) -> U
where
    F: FnOnce(U) -> U,
{
    f(u, u)
    //~^ ERROR this function takes 1 argument but 2 arguments were supplied
}

fn main() {}
