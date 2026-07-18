// Explain how the suggested changes affect receiver-less functions on trait objects.

trait Factory {
    fn create();
}

fn use_factory(_: &dyn Factory) {}
//~^ ERROR the trait `Factory` is not dyn compatible

fn main() {}
