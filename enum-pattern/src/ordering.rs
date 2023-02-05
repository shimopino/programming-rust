#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    fn compute(n: i32, m: i32) -> Ordering {
        if n < m {
            Ordering::Less
        } else if n > m {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    #[test]
    fn compute_test() {
        let result = compute(10, 15);
        assert_eq!(result, Ordering::Less);
    }
}
