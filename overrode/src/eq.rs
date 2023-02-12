use std::cmp::PartialEq;

struct Complex<T> {
    re: T,
    im: T,
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

#[test]
fn test_eq_complex() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };

    assert_eq!(x == y, false);
}
