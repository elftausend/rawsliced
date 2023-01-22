
pub fn ew_binary<T, F>(lhs: &[T], rhs: &[T], out: &mut [T], f: F)
where
    F: Fn(&T, &T) -> T,
{
    for ((lhs, rhs), out) in lhs.iter().zip(rhs).zip(out) {
        *out = f(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::ew_binary;

    #[test]
    fn test_ew_binary() {
        let lhs =     [1, 2, 3, 4, 5, 6, 7];
        let rhs =     [4, 2, 1, 5, 4, 3, 1];
        let mut out = [0, 0, 0, 0, 0, 0, 0];

        ew_binary(&lhs, &rhs, &mut out, |lhs, rhs| lhs + rhs);

        assert_eq!([5, 4, 4, 9, 9, 9, 8], out)
    }
}