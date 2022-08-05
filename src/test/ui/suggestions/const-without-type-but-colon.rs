fn main() {
    const a: = 123;
    //~^ ERROR missing type for `const` item
    const b = 345;
    //~^ ERROR missing type for `const` item

    const c: = "hello";
    //~^ ERROR missing type for `const` item
}
