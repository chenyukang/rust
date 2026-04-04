fn main() {
    assert_eq!();
    //~^ ERROR assert_eq! expects two expressions and an optional panic message

    assert_eq!(1);
    //~^ ERROR assert_eq! expects two expressions and an optional panic message

    assert_eq!(1,);
    //~^ ERROR assert_eq! expects two expressions and an optional panic message

    assert_ne!(1);
    //~^ ERROR assert_ne! expects two expressions and an optional panic message

    assert_ne!(1,);
    //~^ ERROR assert_ne! expects two expressions and an optional panic message

    assert_ne!();
    //~^ ERROR assert_ne! expects two expressions and an optional panic message
}
