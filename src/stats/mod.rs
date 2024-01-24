//! [Criterion]'s statistics library.
//!
//! [Criterion]: https://github.com/bheisler/criterion.rs
//!
//! **WARNING** This library is criterion's implementation detail and there no plans to stabilize
//! it. In other words, the API may break at any time without notice.

#[cfg(test)]
mod test;

pub mod bivariate;
pub mod tuple;
pub mod univariate;

use std::mem;
use std::ops::Deref;

use crate::stats::univariate::Sample;

type A = f64;

/// The bootstrap distribution of some parameter
#[derive(Clone)]
pub struct Distribution<A>(Box<[A]>);

impl Distribution<A> {
    /// Create a distribution from the given values
    pub fn from(values: Box<[A]>) -> Distribution<A> {
        Distribution(values)
    }

    /// Computes the confidence interval of the population parameter using percentiles
    ///
    /// # Panics
    ///
    /// Panics if the `confidence_level` is not in the `(0, 1)` range.
    pub fn confidence_interval(&self, confidence_level: A) -> (A, A) {
        let _0 = 0_f64;
        let _1 = 1_f64;
        let _50 = 50_f64;

        assert!(confidence_level > _0 && confidence_level < _1);

        let percentiles = self.percentiles();

        // FIXME(privacy) this should use the `at_unchecked()` method
        (
            percentiles.at(_50 * (_1 - confidence_level)),
            percentiles.at(_50 * (_1 + confidence_level)),
        )
    }

    /// Computes the "likelihood" of seeing the value `t` or "more extreme" values in the
    /// distribution.
    pub fn p_value(&self, t: A, tails: &Tails) -> A {
        use std::cmp;

        let n = self.0.len();
        let hits = self.0.iter().filter(|&&x| x < t).count();

        let tails = match *tails {
            Tails::One => 1.,
            Tails::Two => 2.,
        };

        cmp::min(hits, n - hits) as f64 / n as f64 * tails
    }
}

impl<A> Deref for Distribution<A> {
    type Target = Sample<A>;

    fn deref(&self) -> &Sample<A> {
        let slice: &[_] = &self.0;

        unsafe { mem::transmute(slice) }
    }
}

/// Number of tails for significance testing
pub enum Tails {
    /// One tailed test
    One,
    /// Two tailed test
    Two,
}

fn dot(xs: &[A], ys: &[A]) -> A {
    xs.iter().zip(ys).fold(0_f64, |acc, (&x, &y)| acc + x * y)
}

fn sum(xs: &[A]) -> A {
    use std::ops::Add;

    xs.iter().cloned().fold(0_f64, Add::add)
}
