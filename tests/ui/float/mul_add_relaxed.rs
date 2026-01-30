//@ run-pass
#![feature(float_mul_add_relaxed)]

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!((*a - *b).abs() < 1.0e-6, "{} is not approximately equal to {}", *a, *b);
    }};
}

fn main() {
    // Test f32::mul_add_relaxed
    {
        let nan: f32 = f32::NAN;
        let inf: f32 = f32::INFINITY;
        let neg_inf: f32 = f32::NEG_INFINITY;

        assert_approx_eq!(1.23_f32.mul_add_relaxed(4.5, 0.67), 6.205);
        assert_approx_eq!((-1.23_f32).mul_add_relaxed(-4.5, -0.67), 4.865);
        assert_approx_eq!(0.0_f32.mul_add_relaxed(8.9, 1.2), 1.2);
        assert_approx_eq!(3.4_f32.mul_add_relaxed(-0.0, 5.6), 5.6);
        assert!(nan.mul_add_relaxed(7.8, 9.0).is_nan());
        assert_eq!(inf.mul_add_relaxed(7.8, 9.0), inf);
        assert_eq!(neg_inf.mul_add_relaxed(7.8, 9.0), neg_inf);
        assert_eq!(8.9_f32.mul_add_relaxed(inf, 3.2), inf);
        assert_eq!((-3.2_f32).mul_add_relaxed(2.4, neg_inf), neg_inf);

        // Basic functionality test
        let m = 10.0_f32;
        let x = 4.0_f32;
        let b = 60.0_f32;
        let result = m.mul_add_relaxed(x, b);
        assert_approx_eq!(result, 100.0);
    }

    // Test f64::mul_add_relaxed
    {
        let nan: f64 = f64::NAN;
        let inf: f64 = f64::INFINITY;
        let neg_inf: f64 = f64::NEG_INFINITY;

        assert_approx_eq!(1.23_f64.mul_add_relaxed(4.5, 0.67), 6.205);
        assert_approx_eq!((-1.23_f64).mul_add_relaxed(-4.5, -0.67), 4.865);
        assert_approx_eq!(0.0_f64.mul_add_relaxed(8.9, 1.2), 1.2);
        assert_approx_eq!(3.4_f64.mul_add_relaxed(-0.0, 5.6), 5.6);
        assert!(nan.mul_add_relaxed(7.8, 9.0).is_nan());
        assert_eq!(inf.mul_add_relaxed(7.8, 9.0), inf);
        assert_eq!(neg_inf.mul_add_relaxed(7.8, 9.0), neg_inf);
        assert_eq!(8.9_f64.mul_add_relaxed(inf, 3.2), inf);
        assert_eq!((-3.2_f64).mul_add_relaxed(2.4, neg_inf), neg_inf);

        // Basic functionality test
        let m = 10.0_f64;
        let x = 4.0_f64;
        let b = 60.0_f64;
        let result = m.mul_add_relaxed(x, b);
        assert_approx_eq!(result, 100.0);
    }
}
