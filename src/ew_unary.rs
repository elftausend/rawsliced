pub fn ew_unary<T, F>(x: &[T], out: &mut [T], f: F)
where
    F: Fn(&T) -> T,
{
    for (out, x) in out.iter_mut().zip(x) {
        *out = f(x)
    }
}

pub fn ew_unary_mut<T, F>(x: &mut [T], f: F)
where
    F: Fn(&T) -> T,
{
    for value in x.iter_mut() {
        *value = f(value);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ew_unary, ew_unary_mut};

    #[test]
    fn test_ew_unary() {
        let x = [1, 2, 3, 4, 5, 6, 7];
        let mut out = [0, 0, 0, 0, 0, 0, 0];

        ew_unary(&x, &mut out, |x| x + 5);

        assert_eq!([6, 7, 8, 9, 10, 11, 12], out)
    }

    #[test]
    fn test_ew_unary_mut() {
        let mut x = [1, 2, 3, 4, 5, 6, 7];

        ew_unary_mut(&mut x, |x| x + 5);

        assert_eq!([6, 7, 8, 9, 10, 11, 12], x)
    }
}
