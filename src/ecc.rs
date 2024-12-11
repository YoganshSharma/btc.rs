use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct FieldElement {
    num: u64,
    prime: u64,
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            // to avoid this maybe we can use a macro for declaring every prime
            panic!("Cannot add two numbers with diffrent fields");
        }
        let num = (self.num + other.num) % self.prime;
        Self {
            num,
            prime:self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot subract two numbers with diffrent fields");
        }
        let num = (self.num - other.num) % self.prime;
        Self {
            num,
            prime:self.prime
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers with diffrent fields");
        }
        let num = (self.num * other.num) % self.prime;
        Self {
            num,
            prime:self.prime
        }
    }
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers with diffrent fields");
        }
        let num = (self.num * mod_pow(other.num, self.prime - 2, self.prime)) % self.prime;
        Self {
            num,
            prime:self.prime
        }
    }
}

impl FieldElement {
    pub fn new(num: u64, prime: u64) -> Self {
        Self {
            num, prime
        }
    }
    pub fn pow(self, exp: u32) -> Self {
        let num = (self.num.pow(exp)) % self.prime;
        Self {
            num,
            prime: self.prime,
        }
    }
}

#[test]
fn test_div() {
    let a = FieldElement::new(5, 31);
    let b = FieldElement::new(18, 31);

    assert_eq!(a/b, FieldElement::new(16, 31));
}
