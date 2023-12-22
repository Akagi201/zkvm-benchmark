#![allow(unused)]
use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};

fn fib_recur(n: BigUint) -> BigUint {
    if n < BigUint::from(2_u32) {
        BigUint::from(n)
    } else {
        fib_recur(n.clone() - 1_u32) + fib_recur(n - 2_u32)
    }
}

fn fib_iter(n: BigUint) -> BigUint {
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    for _ in 0..n.to_u128().unwrap() {
        let tmp = a;
        a = b.clone();
        b = tmp + b;
    }
    a
}

fn fib_i64_mod(n: i64) -> i64 {
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _ in 0..n {
        let tmp: i64 = a;
        a = b;
        b = (tmp + b) % (1<<32);
    }
    a
}

fn main() {
    println!("fib(100) = {}", fib_iter(BigUint::from(100_u32)));
    println!("fib(100)%(1<<32) = {}", fib_iter(BigUint::from(100_u32)).to_u128().unwrap() % (1 << 32));
    println!("fib_i64_mod(100) = {}", fib_i64_mod(100_i64));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_recur() {
        assert_eq!(fib_recur(BigUint::from(0_u32)), BigUint::from(0_u32));
        assert_eq!(fib_recur(BigUint::from(1_u32)), BigUint::from(1_u32));
        assert_eq!(fib_recur(BigUint::from(2_u32)), BigUint::from(1_u32));
        assert_eq!(fib_recur(BigUint::from(3_u32)), BigUint::from(2_u32));
        assert_eq!(fib_recur(BigUint::from(4_u32)), BigUint::from(3_u32));
        assert_eq!(fib_recur(BigUint::from(5_u32)), BigUint::from(5_u32));
        assert_eq!(fib_recur(BigUint::from(6_u32)), BigUint::from(8_u32));
        assert_eq!(fib_recur(BigUint::from(7_u32)), BigUint::from(13_u32));
        assert_eq!(fib_recur(BigUint::from(8_u32)), BigUint::from(21_u32));
        assert_eq!(fib_recur(BigUint::from(9_u32)), BigUint::from(34_u32));
        assert_eq!(fib_recur(BigUint::from(10_u32)), BigUint::from(55_u32));
        assert_eq!(fib_recur(BigUint::from(11_u32)), BigUint::from(89_u32));
        assert_eq!(fib_recur(BigUint::from(12_u32)), BigUint::from(144_u32));
        assert_eq!(fib_recur(BigUint::from(13_u32)), BigUint::from(233_u32));
        assert_eq!(fib_recur(BigUint::from(14_u32)), BigUint::from(377_u32));
        assert_eq!(fib_recur(BigUint::from(15_u32)), BigUint::from(610_u32));
        assert_eq!(fib_recur(BigUint::from(16_u32)), BigUint::from(987_u32));
        assert_eq!(fib_recur(BigUint::from(17_u32)), BigUint::from(1597_u32));
    }

    #[test]
    fn test_fib_iter() {
        assert_eq!(fib_iter(BigUint::from(0_u32)), BigUint::from(0_u32));
        assert_eq!(fib_iter(BigUint::from(1_u32)), BigUint::from(1_u32));
        assert_eq!(fib_iter(BigUint::from(2_u32)), BigUint::from(1_u32));
        assert_eq!(fib_iter(BigUint::from(3_u32)), BigUint::from(2_u32));
        assert_eq!(fib_iter(BigUint::from(4_u32)), BigUint::from(3_u32));
        assert_eq!(fib_iter(BigUint::from(5_u32)), BigUint::from(5_u32));
        assert_eq!(fib_iter(BigUint::from(6_u32)), BigUint::from(8_u32));
        assert_eq!(fib_iter(BigUint::from(7_u32)), BigUint::from(13_u32));
        assert_eq!(fib_iter(BigUint::from(8_u32)), BigUint::from(21_u32));
        assert_eq!(fib_iter(BigUint::from(9_u32)), BigUint::from(34_u32));
        assert_eq!(fib_iter(BigUint::from(10_u32)), BigUint::from(55_u32));
        assert_eq!(fib_iter(BigUint::from(11_u32)), BigUint::from(89_u32));
        assert_eq!(fib_iter(BigUint::from(12_u32)), BigUint::from(144_u32));
        assert_eq!(fib_iter(BigUint::from(13_u32)), BigUint::from(233_u32));
        assert_eq!(fib_iter(BigUint::from(14_u32)), BigUint::from(377_u32));
        assert_eq!(fib_iter(BigUint::from(15_u32)), BigUint::from(610_u32));
        assert_eq!(fib_iter(BigUint::from(16_u32)), BigUint::from(987_u32));
        assert_eq!(fib_iter(BigUint::from(17_u32)), BigUint::from(1597_u32));
    }
}