struct A;
struct B;
struct C;
struct D;

fn f(
    a1: A,
    a2: A,
    b1: B,
    b2: B,
    c1: C,
    c2: C,
) {}

fn main() {
    f(C, A, A, A, B, B, C);
    //f(C, C, A, A, B, B);
    //f(A, A, D, D, B, B); //ok
    //f(C, C, B, B, A, A); 
    //f(C, C, A, B, A, A); 
}


/* 
fn f(a1: A, a2: A, b1: B, c1: C) {}

fn main() {
    f(
        C,
        A,
        A,
        B,
    );
}
*/