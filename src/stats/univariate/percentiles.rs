type A = f64;

/// A "view" into the percentiles of a sample
pub struct Percentiles<A>(Box<[A]>);

// TODO(rust-lang/rfcs#735) move this `impl` into a private percentiles module
impl Percentiles<A> {
    /// Returns the percentile at `p`%
    ///
    /// Safety:
    ///
    /// - Make sure that `p` is in the range `[0, 100]`
    unsafe fn at_unchecked(&self, p: A) -> A {
        let _100 = 100f64;
        debug_assert!(p >= 0f64 && p <= _100);
        debug_assert!(!self.0.is_empty());
        let len = self.0.len() - 1;

        if p == _100 {
            self.0[len]
        } else {
            let rank = (p / _100) * len as f64;
            let integer = rank.floor();
            let fraction = rank - integer;
            let n = integer as usize;
            let &floor = self.0.get_unchecked(n);
            let &ceiling = self.0.get_unchecked(n + 1);

            floor + (ceiling - floor) * fraction
        }
    }

    /// Returns the percentile at `p`%
    ///
    /// # Panics
    ///
    /// Panics if `p` is outside the closed `[0, 100]` range
    pub fn at(&self, p: A) -> A {
        let _0 = 0f64;
        let _100 = 100f64;

        assert!(p >= _0 && p <= _100);
        assert!(!self.0.is_empty());

        unsafe { self.at_unchecked(p) }
    }

    /// Returns the interquartile range
    pub fn iqr(&self) -> A {
        let q1 = self.at(25f64);
        let q3 = self.at(75f64);

        q3 - q1
    }

    /// Returns the 50th percentile
    pub fn median(&self) -> A {
        self.at(50f64)
    }

    /// Returns the 25th, 50th and 75th percentiles
    pub fn quartiles(&self) -> (A, A, A) {
        (self.at(25f64), self.at(50f64), self.at(75f64))
    }
}
