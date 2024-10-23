use std::ops::{Add, Div, Mul, Sub};

pub trait IntegerNumeral:
    Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    fn new(negative: bool, base: u32, values: Vec<u32>) -> Self;
}

#[derive(Debug)]
pub struct BigInt {
    pub negative: bool,
    pub base: u32,
    pub values: Vec<u32>,
}

// Implement the IntegerNumeral trait for BigInt
impl IntegerNumeral for BigInt {
    fn new(negative: bool, base: u32, values: Vec<u32>) -> Self {
        BigInt {
            negative,
            base,
            values,
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Implement addition here
        unimplemented!()
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Implement subtraction here
        unimplemented!()
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Implement multiplication here
        unimplemented!()
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // Implement division here
        unimplemented!()
    }
}

impl IntegerNumeral for u32 {
    fn new(_negative: bool, base: u32, values: Vec<u32>) -> Self {
        values.iter().fold(0, |acc, &x| acc * base + x)
    }
}
