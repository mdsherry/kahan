//! This crate implements a type for computing
//! [Kahan sums](https://en.wikipedia.org/wiki/Kahan_summation_algorithm)
//! over floating point numbers.
//! It also implements a new trait for computing Kahan sums over iterators of floats.


extern crate num_traits;
use num_traits::float::Float;
use std::ops::{Add, AddAssign};
use std::borrow::Borrow;
use std::mem::swap;


#[derive(Debug, Clone)]
pub struct KahanSum<T: Float> {
    sum: T,
    err: T,
}

impl<T: Float> Default for KahanSum<T> {
    fn default() -> Self {
        KahanSum {
            sum: T::zero(),
            err: T::zero(),
        }
    }
}

/// Represents an ongoing Kahan summation. New terms can be added
/// to the sum by simple addition.
///
/// # Examples
///
/// ```
/// # use kahan::KahanSum;
/// let mut kahan_sum = KahanSum::new();
/// kahan_sum += 10000.0f32;
/// kahan_sum += 3.14159f32;
/// assert_eq!(10003.142f32, kahan_sum.sum());
/// assert_eq!(0.000011444092f32, kahan_sum.err());
/// ```
impl<T: Float> KahanSum<T> {
    /// Creates a new `KahanSum` with sum and err initialized to 0
    pub fn new() -> Self {
        KahanSum::default()
    }

    /// Creates a new `KahanSum` with starting sum set to `initial`, but err initalized to 0
    pub fn new_with_value(initial: T) -> Self {
        KahanSum {
            sum: initial,
            err: T::zero(),
        }
    }

    /// Returns the current running sum
    pub fn sum(&self) -> T {
        self.sum
    }

    /// Returns the current error value
    pub fn err(&self) -> T {
        self.err
    }
}


impl<T: Float> AddAssign<T> for KahanSum<T> {
    fn add_assign(&mut self, rhs: T) {
        let mut rhs = rhs;
        if self.sum.abs() < rhs.abs() {
            swap(&mut self.sum, &mut rhs);
        }
        let y = rhs - self.err;
        let sum = self.sum + y;
        let err = (sum - self.sum) - y;
        self.sum = sum;
        self.err = err;
    }
}

impl<T: Float> Add<T> for KahanSum<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let mut rv = self;
        rv += rhs;
        rv
    }
}

pub trait KahanSummator<T: Float> {
    /// Computes the Kahan sum of an iterator.
    /// # Example
    ///
    /// ```
    /// # use kahan::*;
    /// let summands = [10000.0f32, 3.14159, 2.71828];
    /// let kahan_sum = summands.iter().kahan_sum();
    /// assert_eq!(10005.86f32, kahan_sum.sum());
    /// assert_eq!(0.0004813671f32, kahan_sum.err());
    /// ```
    fn kahan_sum(self) -> KahanSum<T>;
}

impl<T, U, V> KahanSummator<T> for U
    where U: Iterator<Item = V>,
          V: Borrow<T>,
          T: Float
{
    fn kahan_sum(self) -> KahanSum<T> {
        self.fold(KahanSum::new(), |sum, item| sum + *item.borrow())
    }
}

#[cfg(test)]
mod tests {
    use ::KahanSummator;
    #[test]
    fn it_works() {
        let summands = [10000.0f32, 3.14159f32, 2.71828f32, 3.14159f32, 2.71828f32, 3.14159f32,
                        2.71828f32];
        // The true sum is 10017.57961. Summing f32s without use of the algorithm
        // gives an answer of 10017.581. Kahan summation gives the more accurate
        // result of 10017.58 with an error of 0.000467. (10017.58 - 0.000467 = 10017.579533)
        assert_eq!(10017.58f32, summands.iter().kahan_sum().sum());

    }
}
