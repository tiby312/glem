#[test]
fn example() {
    use glem::prelude::*;

    let c = glem::rotate_x(0.5).chain(glem::translate(55.0, -5.0, -6.0));

    let c = glem::build(c);

    let x = glem::build(glem::rotate_x(0.5));
    let y = glem::build(glem::translate(55.0, -5.0, -6.0));

    assert_eq!(c, x * y);
}

#[test]
fn inverse_example() {
    let c = glem::combine!(
        glem::rotate_x(0.5),
        glem::rotate_y(0.2),
        glem::rotate_z(0.1),
        glem::translate(55.0, -5.0, -6.0),
        glem::scale(2.0, 4.0, -2.0)
    );

    let c2 = glem::combine!(
        glem::scale(1.0 / 2.0, 1.0 / 4.0, -1.0 / 2.0),
        glem::translate(-55.0, 5.0, 6.0),
        glem::rotate_z(-0.1),
        glem::rotate_y(-0.2),
        glem::rotate_x(-0.5)
    );

    assert_eq!(glem::build_inverse(&c), glem::build(c2));

    approx_eq(&glem::build(&c).inverse(), &glem::build_inverse(c));
}

fn approx_eq(a: &glam::f32::Mat4, b: &glam::f32::Mat4) {
    let a = a.to_cols_array();
    let b = b.to_cols_array();

    for (a, b) in a.into_iter().zip(b.into_iter()) {
        assert!((a - b).abs() < 0.000001, "{}:{}", a, b);
    }
}
