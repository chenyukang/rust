// compile-flags: -Z parse-only

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {}

struct S<T: const usize> { //~ ERROR const bounds must start with `~`
    field: T,
}
//~^ ERROR const bounds must start with `~`

fn main() {

}