// Test for issue #151792
// Check that const params in types are rendered clearly to distinguish from type params
// by displaying them as `const { value }` instead of just `value`.

struct PartialTest<const FOO: bool, const BAR: bool>;

impl PartialTest<true, true> {
    fn build(self) {}
}

impl<const FOO: bool> PartialTest<FOO, false> {
    fn with_bar(self) -> PartialTest<FOO, true> {
        PartialTest
    }
}

fn main() {
    let x: PartialTest<false, true> = PartialTest;
    x.build();
    //~^ ERROR no method named `build` found for struct `PartialTest<const { false }, const { true }>`

    let y: PartialTest<true, true> = PartialTest;
    y.with_bar();
    //~^ ERROR no method named `with_bar` found for struct `PartialTest<const { true }, const { true }>`
}
