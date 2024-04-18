//! This file defines the Model portion of the PGM, which is simply just a
//! linear approximator.
//!
//! NOTE: We are making a simplification and forcing approximation lines
//! to pass through the origin, which slightly degrades performance

use num::PrimInt;

/// A simple linear model for a key-rank segment of data.
#[derive(Clone, Debug)]
pub struct LinearModel<K, const EPSILON: usize> {
    /// Define the approximation line. See note at top of file about forcing
    /// approximations to pass through the origin.
    pub key: K,
    pub slope: f64,

    /// How many entries are indexed by this model. Not strictly needed but
    /// useful for debugging.
    pub size: usize,
}

impl<K: PrimInt, const EPSILON: usize> LinearModel<K, EPSILON> {
    /// Construct a new model from the smallest key, slope, and size
    pub fn new(key: K, slope: f64, size: usize) -> Self {
        debug_assert!(slope.is_normal());
        Self { key, slope, size }
    }

    /// Approximation logic for linear models
    pub fn approximate(&self, key: &K) -> (usize, usize) {
        let run = num::cast::<K, f64>(key.clone().saturating_sub(self.key)).unwrap();
        let pos = (run * self.slope).floor() as i64;
        let pos = pos.max(0) as usize;

        (pos.saturating_sub(EPSILON), pos + EPSILON + 2)
    }

    /// Construct a sentinel model which will sit at the end of a layer
    pub fn sentinel() -> Self {
        Self {
            key: K::max_value(),
            slope: 0.0,
            size: 0,
        }
    }

    pub fn min_key(&self) -> K {
        self.key
    }
}

/// Simple component with simple test(s)
#[cfg(test)]
mod pgm_model_tests {
    use super::*;

    #[test]
    fn pgm_model_basic() {
        const EPS: usize = 2;
        let key: usize = 10;
        let slope: f64 = 1.0;
        let slope_usize: usize = 1;
        let model: LinearModel<usize, EPS> = LinearModel::new(key, slope, 6);

        for test in 20..1000 {
            let test: usize = test;
            let approx = model.approximate(&test);
            let expected_lo = (test - key) * slope_usize - EPS;
            let expected_hi = expected_lo + EPS * 2 + 2;

            assert!(approx.0 == expected_lo);
            assert!(approx.1 == expected_hi);
        }
    }
}
