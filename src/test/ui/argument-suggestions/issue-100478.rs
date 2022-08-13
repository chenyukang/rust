use std::sync::Arc;
macro_rules! GenT {
    ($name:tt) => {
        #[derive(Default, Debug)]
        struct $name {
            #[allow(unused)]
            val: i32,
        }

        impl $name {
            #[allow(unused)]
            fn new(val: i32) -> Self {
                $name { val }
            }
        }
    };
}

GenT!(T1);
GenT!(T2);
GenT!(T3);
GenT!(T4);
GenT!(T5);
GenT!(T6);
GenT!(T7);
GenT!(T8);
GenT!(T9);
GenT!(T10);
GenT!(T11);

#[allow(unused)]
fn foo(
    p1: T1,
    p2: Arc<T2>,
    p3: T3,
    p4: Arc<T4>,
    p5: T5,
    p6: T6,
    p7: T7,
    p8: Arc<T8>,
    p9: T9,
    p10: T10,
    p11: T11,
) {
}

fn main() {
    let p1 = T1::new(0);
    let p2 = Arc::new(T2::new(0));
    let p3 = T3::new(0);
    let p4 = Arc::new(T4::new(1));
    let p5 = T5::new(0);
    let p6 = T6::new(0);
    let p7 = T7::new(0);
    let p8 = Arc::default();
    let p9 = T9::new(0);
    let p10 = T10::new(0);
    let p11 = T11::new(0);

    foo(
        //~^ ERROR 60:5: 60:8: this function takes 11 arguments but 10 arguments were supplied [E0061]
        p1, //p2,
        p3, p4, p5, p6, p7, p8, p9, p10, p11,
    );
}
