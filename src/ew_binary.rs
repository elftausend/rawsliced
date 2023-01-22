
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
        let lhs = [1, 2, 3, 4, 5, 6, 7];
        let rhs = [4, 2, 1, 5, 4, 3, 1];
        let mut out = [0, 0, 0, 0, 0, 0, 0];

        ew_binary(&lhs, &rhs, &mut out, |lhs, rhs| lhs + rhs);

        assert_eq!([5, 4, 4, 9, 9, 9, 8], out)
    }

    #[cfg(feature = "rayon")]
    use rayon::prelude::*;

    #[cfg(feature = "rayon")]
    pub fn ew_binary_rayon<T: Send + Sync, F>(lhs: &[T], rhs: &[T], out: &mut [T], f: F)
    where
        F: Fn(&T, &T) -> T + Sync,
    {
        lhs.par_iter()
            .zip(rhs)
            .zip(out)
            .for_each(|((lhs, rhs), out)| *out = f(lhs, rhs));
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn test_ew_binary_speed() {

        let lhs = vec![1.1; 20221];
        let rhs = vec![1.1; 20221];

        let start = std::time::Instant::now();

        (0..1000).into_par_iter().for_each(|_| {
            let mut out = vec![0.; 20221];
            ew_binary(&lhs, &rhs, &mut out, |lhs, rhs| lhs + rhs);
            assert_eq!(out, vec![2.2; 20221]);
        });

        println!("rayon elapsed: {:?}", start.elapsed());

        let start = std::time::Instant::now();

        for _ in 0..1000 {
            let mut out = vec![0.; 20221];

            ew_binary_rayon(&lhs, &rhs, &mut out, |lhs, rhs| lhs + rhs);
            assert_eq!(out, vec![2.2; 20221]);
        }

        println!("elapsed: {:?}", start.elapsed());
    }
}
