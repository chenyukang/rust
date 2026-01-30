// Test for issue #151796
// Allow `#[diagnostic::on_unimplemented]` on inherent `impl`s

struct PartialTest<const FOO: bool, const BAR: bool> {
    value: u32,
}

impl PartialTest<false, false> {
    fn new() -> Self {
        PartialTest { value: 0 }
    }
}

// This allows customizing the error message when `build` method is not found
#[diagnostic::on_unimplemented(
    message = "all fields must be initialized before `Test` can be built",
    label = "not all fields are initialized",
    note = "call `with_foo` and `with_bar` before calling `build`"
)]
impl PartialTest<true, true> {
    fn build(self) -> u32 {
        self.value
    }
}

#[diagnostic::on_unimplemented(
    message = "the same field can't be set twice",
    label = "method can't be called twice"
)]
impl<const FOO: bool> PartialTest<FOO, false> {
    fn with_bar(self, bar: u32) -> PartialTest<FOO, true> {
        PartialTest { value: self.value + bar }
    }
}

impl<const BAR: bool> PartialTest<false, BAR> {
    fn with_foo(self, foo: u32) -> PartialTest<true, BAR> {
        PartialTest { value: self.value + foo }
    }
}

fn main() {
    // This should trigger the customized label from the `with_bar` impl
    let _ = PartialTest::new().with_bar(1).with_foo(1).with_bar(2);
    //~^ ERROR no method named `with_bar` found
}
