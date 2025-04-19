// Copyright (c) 2025 Richard Sun
// Licensed under the MIT License (https://opensource.org/licenses/MIT)

use std::{
    cmp::Ordering, fmt::{self, Display, Formatter}, hash::{Hash, Hasher}, i32, ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign 
    }
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Normal,
    Infinity,
    NegInfinity,
    Zero,
    NaN
}

#[derive(Debug)]
pub enum ConversionError {
    OutOfRangeError, 
    NaNConversion, 
    InfiniteConversion, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    nume: i32,
    deno: i32, 
    frac_type: Type, 
}

impl Fraction {
    pub const INFINITY: Self = Self { nume: i32::MAX, deno: 1, frac_type: Type::Infinity };
    pub const NEG_INFINITY: Self = Self { nume: i32::MIN, deno: 1, frac_type: Type::NegInfinity };
    pub const NAN: Self = Self { nume: 0, deno: 0, frac_type: Type::NaN };
    pub const ZERO: Self = Self { nume: 0, deno: 1, frac_type: Type::Zero };

    pub const MAX: Self = Self { nume: i32::MAX - 1, deno: 1, frac_type: Type::Normal};
    pub const MIN: Self = Self { nume: i32::MIN + 1, deno: 1, frac_type: Type::Normal};
    pub const MIN_POSITIVE: Self = Self { nume: 1, deno: i32::MAX, frac_type: Type::Normal};

    const LIMITER: u64 = i32::MAX as u64;

    // new
    pub fn new(nume: i32, deno: i32) -> Self {
        let frac_type = Self::determine_frac_type(nume, deno);
        match frac_type {
            Type::Infinity => Self::INFINITY, 
            Type::NegInfinity => Self::NEG_INFINITY, 
            Type::NaN => Self::NAN, 
            Type::Zero => Self::ZERO, 
            Type::Normal => {
                let sign = nume.signum() * deno.signum();
        
                let (nume, deno) = (nume.unsigned_abs() as u64, deno.unsigned_abs() as u64);
                let gcd_val = Self::gcd(nume, deno);
                let (nume, deno) = Self::shrink(nume / gcd_val, deno / gcd_val);
                let (nume, deno) = (nume as i32 * sign, deno as i32);
        
                Self {
                    nume, 
                    deno, 
                    frac_type: Self::determine_frac_type(nume, deno)
                }
            }
        }
    }

    // determine num type
    fn determine_frac_type(nume: i32, deno: i32) -> Type {
        if deno == 0 { 
            match nume.signum() {
                1 => Type::Infinity,
                -1 => Type::NegInfinity,
                _ => Type::NaN,
            }
        } else if nume == 0 {
            Type::Zero
        } else if deno == 1 {
            match nume {
                i32::MAX => Type::Infinity,
                i32::MIN => Type::NegInfinity,
                _ => Type::Normal,
            }
        } else {
            Type::Normal
        }
    }

    fn gcd<T>(a: T, b: T) -> T
    where
        T: Rem<Output = T> + From<u8> + Eq + Copy // + std::fmt::Display
    {
        let (mut a, mut b) = (a, b);
        while b != T::from(0u8) {
            (a, b) = (b, a % b);
        };
        a
    }

    fn lcm<T>(a: T, b: T) -> (T, T, T)
    where
        T: Div<Output = T> + Rem<Output = T> + From<u8> + Eq + Copy // + std::fmt::Display
    {
        let gcd = Self::gcd(a, b);
        (b / gcd, a / gcd, gcd)
    }

    fn shrink(nume: u64, deno: u64) -> (u32, u32) {
        let nume_abs = nume;
        let deno_abs = deno;
            
        if nume_abs <= Self::LIMITER && deno_abs <= Self::LIMITER {
            return (nume as u32, deno as u32);
        }

        let (mut p_0, mut q_0, mut p_1, mut q_1) = (0, 1, 1, 0); // [0, +inf)
        let (mut nume, mut deno) = (nume_abs, deno_abs);
        loop {
            let q = nume / deno;
            let p_2 = p_0 + q * p_1;
            let q_2 = q_0 + q * q_1;
            
            if p_2 > Self::LIMITER || q_2 > Self::LIMITER {
                break;
            }

            (p_0, q_0, p_1, q_1) = (p_1, q_1, p_2, q_2);
            (nume, deno) = (deno, nume - q * deno);
        }
        let (k_q, k_p) = {
            let k_q = if q_1 != 0 {
                (Self::LIMITER - q_0) / q_1
            } else {
                return (i32::MAX as u32, 1); // q_1 == 0 <=> inf
            };
        
            let k_p = if p_1 != 0 {
                (Self::LIMITER - p_0) / p_1
            } else {
                return (0, 1); // p_1 == 0 <=> 0
            };
        
            (k_q, k_p)
        };
        let k = k_q.min(k_p).max(0);

        let (nume_1, deno_1) = (p_1, q_1);
        let (nume_2, deno_2) = (p_0 + k * p_1, q_0 + k * q_1);
        
        let d_1 = (nume_1 as i128 * deno_abs as i128 - nume_abs as i128 * deno_1 as i128).abs();
        let d_2 = (nume_2 as i128 * deno_abs as i128 - nume_abs as i128 * deno_2 as i128).abs();

        if d_1 * deno_2 as i128 <= d_2 * deno_1 as i128 { (nume_1 as u32, deno_1 as u32) } else { (nume_2 as u32, deno_2 as u32) }
    }

    pub fn sign(&self) -> Self {
        match self.frac_type {
            Type::NaN => Self::NAN,
            Type::Zero => Self::ZERO,
            Type::Infinity => Self { 
                nume: 1, 
                deno: 1, 
                frac_type: Type::Normal
            },
            Type::NegInfinity => Self { 
                nume: -1, 
                deno: 1, 
                frac_type: Type::Normal
            },
            Type::Normal => if self.nume >= 0 { Self { 
                nume: 1, 
                deno: 1, 
                frac_type: Type::Normal
            } } else { Self { 
                nume: -1, 
                deno: 1, 
                frac_type: Type::Normal
            } }
        }
    }
    
    fn i32_sign(&self) -> i32 {
        match self.frac_type {
            Type::NaN => 0,
            Type::Zero => 0,
            Type::Infinity => 1,
            Type::NegInfinity => -1,
            Type::Normal => if self.nume > 0 { 1 } else { -1 }
        }
    }

    pub fn is_positive(&self) -> bool {
        match self.frac_type {
            Type::NegInfinity | Type::NaN | Type::Zero => false, 
            Type::Infinity => true,
            _ => self.nume > 0
        }
    }

    pub fn is_negative(&self) -> bool {
        match self.frac_type {
            Type::Infinity | Type::NaN | Type::Zero => false, 
            Type::NegInfinity => true,
            _ => self.nume < 0
        }
    }

    pub fn is_zero(&self) -> bool {
        self.frac_type == Type::Zero
    }

    pub fn is_infinity(&self) -> bool {
        self.frac_type == Type::Infinity
    }

    pub fn is_neg_infinity(&self) -> bool {
        self.frac_type == Type::NegInfinity
    }

    pub fn is_nan(&self) -> bool {
        self.frac_type == Type::NaN
    }

    pub fn is_normal(&self) -> bool {
        self.frac_type == Type::Normal
    }

    pub fn abs(&self) -> Self {
        match self.frac_type {
            Type::NegInfinity | Type::Infinity => Self::INFINITY, 
            Type::NaN => Self::NAN, 
            Type::Zero => Self::ZERO, 
            _ => Self {
                nume: self.nume.abs(), 
                deno: self.deno, 
                frac_type: self.frac_type
            }
        }
    }

    pub fn reciprocal(&self) -> Self {
        match self.frac_type {
            Type::Infinity => Self::ZERO, 
            Type::NegInfinity => Self::ZERO, 
            Type::NaN => Self::NAN, 
            Type::Zero => Self::INFINITY, 
            _ => Self { 
                nume: self.deno.abs() * self.i32_sign(), 
                deno: self.nume.abs(), 
                frac_type: self.frac_type
            }
        }
    }

    // operations
    fn get_add_type(self, rhs: Self) -> Type {
        match (self.frac_type, rhs.frac_type) {
            // NaN
            (Type::NaN, _) | (_, Type::NaN) => Type::NaN,
            
            // (+inf / -inf) + (+inf / -inf)
            (Type::Infinity, Type::NegInfinity) | (Type::NegInfinity, Type::Infinity) => Type::NaN,
            (Type::Infinity, _) | (_, Type::Infinity) => Type::Infinity,
            (Type::NegInfinity, _) | (_, Type::NegInfinity) => Type::NegInfinity,
            
            // 0 + 0
            (Type::Zero, Type::Zero) => Type::Zero,
            
            // normal + normal
            _ => Type::Normal, 
        }
    }

    fn normal_add(self, rhs: Self) -> (i32, i32) {
        let (a, b) = (self.nume as i64, self.deno as i64);
        let (c, d) = (rhs.nume as i64, rhs.deno as i64);

        let (e, f, gcd_bd) = Self::lcm(b, d);
        let (nume, deno) = (
            a * e + c * f, e * f * gcd_bd
        );

        let sign = nume.signum() as i32;
        let (u_num, u_den) = (nume.unsigned_abs(), deno as u64);

        let gcd = Self::gcd(u_num, u_den);
        let (simplified_num, simplified_den) = (u_num / gcd, u_den / gcd);

        let (num, den) = Self::shrink(simplified_num, simplified_den);

        (num as i32 * sign, den as i32)
    }

    fn get_mul_type(self, rhs: Self) -> Type {
        match (self.frac_type, rhs.frac_type) {
            // NaN
            (Type::NaN, _) | (_, Type::NaN) => Type::NaN,
        
            // inf * zero
            (Type::Infinity | Type::NegInfinity, Type::Zero) |
            (Type::Zero, Type::Infinity | Type::NegInfinity) => Type::NaN,
        
            // inf * inf | -inf * -inf
            (Type::Infinity, Type::Infinity) | (Type::NegInfinity, Type::NegInfinity) => Type::Infinity,
        
            // inf * -inf | -inf * inf
            (Type::Infinity, Type::NegInfinity) | (Type::NegInfinity, Type::Infinity) => Type::NegInfinity,
        
            // 0 * 0/normal | 0/normal * 0
            (Type::Zero, _) | (_, Type::Zero) => Type::Zero,
        
            // normal * inf / normal * inf 
            (Type::Normal, Type::Infinity) | (Type::Infinity, Type::Normal) => 
                if self.is_negative() ^ rhs.is_negative() { Type::NegInfinity } else { Type::Infinity },
            (Type::Normal, Type::NegInfinity) | (Type::NegInfinity, Type::Normal) => 
                if self.is_negative() ^ rhs.is_negative() { Type::Infinity } else { Type::NegInfinity }, 

            // normal * normal
            (Type::Normal, Type::Normal) => Type::Normal
        }

    }

    fn normal_mul(self, rhs: Self) -> (i32, i32) {
        let (a, b) = (self.nume.unsigned_abs() as u64, self.deno.unsigned_abs() as u64);
        let (c, d) = (rhs.nume.unsigned_abs() as u64, rhs.deno.unsigned_abs() as u64);

        let gcd_ad = Self::gcd(a, d);
        let gcd_bc = Self::gcd(b, c);
        let a = a / gcd_ad;
        let d = d / gcd_ad;
        let b = b / gcd_bc;
        let c = c / gcd_bc;

        let (nume, deno) = (a * c, b * d);
        // println!("mul_impl {} {}", nume, deno);
        let (nume, deno) = Self::shrink(nume, deno);
        
        (nume as i32 * self.i32_sign() * rhs.i32_sign(), deno as i32)
    }
}

impl<T: Into<Fraction>> Add<T> for Fraction {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        let add_type = self.get_add_type(rhs);
        match add_type {
            Type::Infinity => Self::INFINITY, 
            Type::NegInfinity => Self::NEG_INFINITY, 
            Type::NaN => Self::NAN, 
            Type::Zero => Self::ZERO, 
            Type::Normal => {
                let (nume, deno) = self.normal_add(rhs);
                Self { 
                    nume, 
                    deno, 
                    frac_type: Self::determine_frac_type(nume, deno)
                }
            }
        }
    }
}

impl<T: Into<Fraction>> Sub<T> for Fraction {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Self = -rhs.into();
        self + rhs
    }
}

impl<T: Into<Fraction>> Mul<T> for Fraction {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        let add_type = self.get_mul_type(rhs);
        match add_type {
            Type::Infinity => Self::INFINITY, 
            Type::NegInfinity => Self::NEG_INFINITY, 
            Type::NaN => Self::NAN, 
            Type::Zero => Self::ZERO, 
            Type::Normal => {
                let (nume, deno) = self.normal_mul(rhs);
                Self { 
                    nume, 
                    deno, 
                    frac_type: Self::determine_frac_type(nume, deno)
                }
            }
        }
    }
}

impl<T: Into<Fraction>> Div<T> for Fraction {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into().reciprocal();
        self * rhs
    }
}

impl<T: Into<Fraction>> AddAssign<T> for Fraction {
    fn add_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        let add_type = self.get_add_type(rhs);
        match add_type {
            Type::Infinity => *self = Self::INFINITY, 
            Type::NegInfinity => *self = Self::NEG_INFINITY, 
            Type::NaN => *self = Self::NAN, 
            Type::Zero => *self = Self::ZERO, 
            Type::Normal => {
                (self.nume, self.deno) = self.normal_add(rhs);
                self.frac_type = Self::determine_frac_type(self.nume, self.deno);
            }
        }
    }
}

impl<T: Into<Fraction>> SubAssign<T> for Fraction {
    fn sub_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        *self += -rhs;
    }
}

impl<T: Into<Fraction>> MulAssign<T> for Fraction {
    fn mul_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        let add_type = self.get_add_type(rhs);
        match add_type {
            Type::Infinity => *self = Self::INFINITY, 
            Type::NegInfinity => *self = Self::NEG_INFINITY, 
            Type::NaN => *self = Self::NAN, 
            Type::Zero => *self = Self::ZERO, 
            Type::Normal => {
                (self.nume, self.deno) = self.normal_mul(rhs);
                self.frac_type = Self::determine_frac_type(self.nume, self.deno);
            }
        }
    }
}

impl<T: Into<Fraction>> DivAssign<T> for Fraction {
    fn div_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        *self *= rhs.reciprocal();
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.frac_type {
            Type::Infinity => Self::NEG_INFINITY, 
            Type::NegInfinity => Self::INFINITY, 
            Type::NaN => Self::NAN, 
            Type::Zero => Self::ZERO, 
            Type::Normal => {
                Self {
                    nume: -self.nume, 
                    deno: self.deno, 
                    frac_type: Type::Normal
                }
            }
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.frac_type {
            Type::Infinity => write!(f, "inf"), 
            Type::NegInfinity => write!(f, "-inf"), 
            Type::NaN => write!(f, "nan"), 
            Type::Zero => write!(f, "0"), 
            Type::Normal => if self.deno == 1 {
                write!(f, "{}", self.nume)
            } else {
                write!(f, "{}/{}", self.nume, self.deno)
            }
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.frac_type, other.frac_type) {
            (Type::NaN, _) | (_, Type::NaN) => None, 

            // self_type: infinity
            (Type::Infinity, Type::NegInfinity) | (Type::Infinity, Type::Normal) |
            (Type::Infinity, Type::Zero) => Some(Ordering::Greater),  
            (Type::Infinity, Type::Infinity) => Some(Ordering::Equal), 

            // self_type: neg_infinity
            (Type::NegInfinity, Type::Infinity) | (Type::NegInfinity, Type::Normal) |
            (Type::NegInfinity, Type::Zero) => Some(Ordering::Less), 
            (Type::NegInfinity, Type::NegInfinity) => Some(Ordering::Equal), 

            // self_type: normal
            (Type::Normal, Type::Infinity) => Some(Ordering::Less), 
            (Type::Normal, Type::NegInfinity) => Some(Ordering::Less), 
            (Type::Normal, _) | (Type::Zero, _) => {
                let (a, b) = (self.nume as i64, self.deno as i64);
                let (c, d) = (other.nume as i64, other.deno as i64);
                Some((a * d).cmp(&(b * c)))
            }
        }
    }
}

// panic! when NaN
// impl Ord for Fraction {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }


macro_rules! impl_from_safe {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Fraction {
                fn from(value: $t) -> Self {
                    let value = value as i32;
                    match Self::determine_frac_type(value, 1) {
                        Type::Zero => Self::ZERO, 
                        Type::Normal => Self {
                            nume: value, 
                            deno: 1, 
                            frac_type: Type::Normal
                        }, 
                        _ => Self::NAN
                    }
                }
            }
        )*
    };
}

macro_rules! impl_from_unsigned_unsafe {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Fraction {
                fn from(value: $t) -> Self {
                    let value = value as i32;
                    match Self::determine_frac_type(value, 1) {
                        Type::Zero => Self::ZERO, 
                        Type::Infinity => Self::INFINITY, 
                        Type::Normal => Self {
                            nume: value, 
                            deno: 1, 
                            frac_type: Type::Normal
                        }, 
                        _ => Self::NAN
                    }
                }
            }
        )*
    };
}

macro_rules! impl_from_signed_unsafe {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Fraction {
                fn from(value: $t) -> Self {
                    let value = value as i32;
                    match Self::determine_frac_type(value, 1) {
                        Type::Zero => Self::ZERO, 
                        Type::Infinity => Self::INFINITY, 
                        Type::NegInfinity => Self::NEG_INFINITY, 
                        Type::Normal => Self {
                            nume: value, 
                            deno: 1, 
                            frac_type: Type::Normal
                        }, 
                        _ => Self::NAN
                    }
                }
            }
        )*
    };
}

impl_from_safe!(u8, u16, i8, i16);
impl_from_unsigned_unsafe!(i32, u32, u64, u128);
impl_from_signed_unsafe!(i64, i128);

macro_rules! impl_from_for_float {
    ($($t:ty),*) => {
        $(
            impl From<Fraction> for $t {
                fn from(value: Fraction) -> Self {
                    match value.frac_type {
                        Type::Infinity => <$t>::INFINITY, 
                        Type::NegInfinity => <$t>::NEG_INFINITY, 
                        Type::NaN => <$t>::NAN, 
                        Type::Zero => 0.0, 
                        _ => value.nume as $t / value.deno as $t
                    }
                }
            }
        )*
    };
}

impl_from_for_float!(f32, f64);

macro_rules! impl_try_from_for_integer_with_lower_capacity {
    ($($t:ty),*) => {
        $(
            impl TryFrom<Fraction> for $t {
                type Error = ConversionError;

                fn try_from(value: Fraction) -> Result<Self, Self::Error> {
                    match value.frac_type {
                        Type::Infinity | Type::NegInfinity => Err(ConversionError::InfiniteConversion), 
                        Type::NaN => Err(ConversionError::NaNConversion), 
                        Type::Zero => Ok(0), 
                        Type::Normal => {
                            let integer = value.nume / value.deno;
                            if Self::MIN as i32 <= integer && integer <= Self::MAX as i32 {
                                Ok(integer as $t)
                            } else {
                                Err(ConversionError::OutOfRangeError)
                            }
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! impl_try_from_for_unsigned_integer_with_greater_capacity {
    ($($t:ty),*) => {
        $(
            impl TryFrom<Fraction> for $t {
                type Error = ConversionError;

                fn try_from(value: Fraction) -> Result<Self, Self::Error> {
                    match value.frac_type {
                        Type::Infinity | Type::NegInfinity => Err(ConversionError::InfiniteConversion), 
                        Type::NaN => Err(ConversionError::NaNConversion), 
                        Type::Zero => Ok(0), 
                        Type::Normal => {
                            let integer = value.nume / value.deno;
                            if integer >= 0 {
                                Ok(integer as $t)
                            } else {
                                Err(ConversionError::OutOfRangeError)
                            }
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! impl_try_from_for_signed_integer_with_greater_capacity {
    ($($t:ty),*) => {
        $(
            impl TryFrom<Fraction> for $t {
                type Error = ConversionError;

                fn try_from(value: Fraction) -> Result<Self, Self::Error> {
                    match value.frac_type {
                        Type::Infinity | Type::NegInfinity => Err(ConversionError::InfiniteConversion), 
                        Type::NaN => Err(ConversionError::NaNConversion), 
                        Type::Zero => Ok(0), 
                        Type::Normal => {
                            let integer = value.nume / value.deno;
                            Ok(integer as $t)
                        }
                    }
                }
            }
        )*
    };
}

impl_try_from_for_integer_with_lower_capacity!(i8, i16, u8, u16);
impl_try_from_for_unsigned_integer_with_greater_capacity!(u32, u64, u128);
impl_try_from_for_signed_integer_with_greater_capacity!(i32, i64, i128);

impl Hash for Fraction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nume.hash(state);
        self.deno.hash(state);
    }
}