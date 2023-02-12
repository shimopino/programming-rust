use std::ops::Add;

struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::Complex;

    #[test]
    fn test_complex_add() {
        let comp1 = Complex { re: 1, im: 10 };
        let comp2 = Complex { re: 100, im: 5 };

        let result = comp1.add(comp2);

        assert_eq!(result.re, 101);
        assert_eq!(result.im, 15);
    }
}
