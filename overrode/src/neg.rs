use std::ops::Neg;

struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

#[test]
fn test_neg_complex() {
    let comp1 = Complex { re: 10, im: 20 };

    let result = &comp1;

    assert_eq!(-1 * comp1.re, -result.re);
    assert_eq!(-1 * comp1.im, -result.im);
}
