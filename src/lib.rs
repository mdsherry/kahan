extern crate num;
use num::traits::float::Float;
use std::ops::{Add, AddAssign};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct KahanSum<T: Float> {
    sum: T,
    err: T,
}

impl<T: Float> KahanSum<T> {
    pub fn new() -> Self {
        KahanSum {
            sum: T::zero(),
            err: T::zero(),
        }
    }

    pub fn new_with_value(initial: T) -> Self {
        KahanSum {
            sum: initial,
            err: T::zero(),
        }
    }

    pub fn sum(&self) -> T {
        self.sum
    }

    pub fn err(&self) -> T {
        self.err
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

trait KahanSummator<T: Float> {
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

impl<T: Float> AddAssign<T> for KahanSum<T> {
    fn add_assign(&mut self, rhs: T) {
        let y = rhs - self.err;
        let sum = self.sum + y;
        let err = (sum - self.sum) - y;
        self.sum = sum;
        self.err = err;
    }
}

#[cfg(test)]
mod tests {
    use ::KahanSummator;
    #[test]
    fn it_works() {
        let summands = [10000.0f32, 3.14159f32, 2.71828f32, 3.14159f32, 2.71828f32, 3.14159f32, 2.71828f32];
        // The true sum is 10017.57961. Summing f32s without use of the algorithm
        // gives an answer of 10017.581. Kahan summation gives the more accurate
        // result of 10017.58 with an error of 0.000467. (10017.58 - 0.000467 = 10017.579533)
        assert_eq!(10017.58f32, summands.iter().kahan_sum().sum());
        
    }
}
