/// An element wise assign operation.
/// `lhs` is modified by `rhs` somehow.
/// # Example
/// ```
/// let mut lhs = [1, 2, 3, 4, 5, 6, 7];
/// let rhs =     [4, 2, 1, 5, 4, 3, 1];
///
/// sliced::ew_assign_binary(&mut lhs, &rhs, |lhs, rhs| *lhs -= rhs);
///
/// assert_eq!([-3, 0, 2, -1, 1, 3, 6], lhs)
/// ```
pub fn ew_assign_binary<T, F>(lhs: &mut [T], rhs: &[T], f: F)
where
    F: Fn(&mut T, &T),
{
    for (lhs, rhs) in lhs.iter_mut().zip(rhs) {
        f(lhs, rhs)
    }
}

// ew_unary_mut
pub fn ew_assign_scalar<T, F>(lhs: &mut [T], rhs: &T, f: F)
where
    F: Fn(&mut T, &T),
{
    for value in lhs {
        f(value, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ew_assign_binary, ew_assign_scalar};

    #[test]
    fn test_ew_assign_binary() {
        let mut lhs = [1, 2, 3, 4, 5, 6, 7];
        let rhs = [4, 2, 1, 5, 4, 3, 1];

        ew_assign_binary(&mut lhs, &rhs, |lhs, rhs| *lhs += rhs);

        assert_eq!([5, 4, 4, 9, 9, 9, 8], lhs)
    }

    #[test]
    fn test_ew_assign_scalar() {
        let mut lhs = [1, 2, 3, 4, 5, 6, 7];

        ew_assign_scalar(&mut lhs, &3, |lhs, rhs| *lhs += rhs);

        assert_eq!([4, 5, 6, 7, 8, 9, 10], lhs)
    }
}
