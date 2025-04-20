mod fraction;

fn main() {
    use fraction::Fraction;
    // let a = Fraction::new(50i32, 10i32);
    // let b = Fraction::new(-50i32, 17i32);
    // let c = Fraction::new(0, 1);

    // println!("{} {} {}", a, b, c);
    // println!("{} {} {}", a.is_positive(), a.is_negative(), a.is_zero());
    // println!("{} {} {}", b.is_positive(), b.is_negative(), b.is_zero());
    // println!("{} {} {}", c.is_positive(), c.is_negative(), c.is_zero());
    // println!("{} {} {}", a.abs(), b.abs(), c.abs());
    // println!("{} {} {}", a.sign(), b.sign(), c.sign());
    // println!("{} {} {}", a > b, b < c, a <= c);
    // println!("{}", a + b);
    // let n = Fraction::new(50, 1);
    // let a = sqrt(&n);
    // println!("{} {}", n, a);
    // let n = Fraction::new(100000000, 1);
    // let a = sqrt(&n);
    // println!("{} {}", n, a);

    // println!("{:?}", Fraction::new(1, -1));
    // println!("{:?}", Fraction::new(-1, 1));
    // println!("{:?}", Fraction::new(1, -2));
    // println!("{:?}", Fraction::new(-1, -2));

    // println!("{}", Fraction::new(1, 0));
    // println!("{}", Fraction::new(1, 2) / Fraction::new(2, 1));
    // let big = Fraction::new(5, i32::MAX);
    // let bigger = big / 2;
    // println!("{} {}", big, bigger);

    // println!("{}", Fraction::new(891980073, 2144729028));
    // println!("{}", sqrt_fraction(Fraction::new(891980073, 2144729028)).unwrap());
    
    // println!("{}", sqrt_fraction(Fraction::new(i32::MAX, 1)));

    // let n = Fraction::new(2147483647, 4);
    // let prev = (n + 1) / 2;
    // println!("{}", n + 2);
    // println!("{}", prev);
    // println!("{}", n / prev);
    // println!("{}", (n / prev) + prev);
    // println!("{}", ((n / prev) + prev) / 2);
    // let n = Fraction::from(i32::MAX - 2);
    // let s = sqrt_fraction(n);

    fn sqrt(n: Fraction) -> Option<Fraction> {
        let mut prev;
        let mut curr;

        if n.is_negative() {
            return None;
        } else if n.is_zero() {
            return Some(Fraction::ZERO);
        } else if (n - 1).is_positive() {
            prev = (n + 1) / 2;
        } else {
            prev = Fraction::from(1);
        }

        curr = (n / prev + prev) / 2;
        loop {
            if curr - prev == Fraction::ZERO {
                println!("{} {}", curr, prev);
                return Some(curr);
            }
            prev = curr;
            curr = (n / prev + prev) / 2;
        }
    }
    
    sqrt(Fraction::from(39));
}

#[cfg(test)]
mod tests {
    use crate::fraction::{Fraction, ConversionError};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use rand;

    pub fn sqrt_fraction(n: Fraction) -> Option<Fraction> {
        let mut prev;
        let mut curr;
        let zero = Fraction::from(0);
        if n.is_negative() {
            return None;
        } else if n.is_zero() {
            return Some(Fraction::from(0));
        } else if (n - 1).is_positive() {
            prev = (n + 1) / 2;
        } else {
            prev = Fraction::from(1);
        }

        curr = (n / prev + prev) / 2;
        loop {
            if curr - prev == zero {
                return Some(curr);
            }
            prev = curr;
            curr = (n / prev + prev) / 2;
        }
    }

    #[test]
    fn test_creation_and_reduction() {
        let f = Fraction::new(4, 6);
        assert_eq!(f, Fraction::new(2, 3));

        let f = Fraction::new(-3, 6);
        assert_eq!(f, Fraction::new(-1, 2));

        let f = Fraction::new(3, -6);
        assert_eq!(f, Fraction::new(-1, 2));

        let f = Fraction::new(0, 5);
        assert_eq!(f, Fraction::new(0, 1));
    }

    #[test]
    fn test_arithmetic_operations() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);
        assert_eq!(a + b, Fraction::new(5, 6));

        let a = Fraction::new(3, 4);
        let b = Fraction::new(1, 4);
        assert_eq!(a - b, Fraction::new(1, 2));

        let a = Fraction::new(2, 3);
        let b = Fraction::new(3, 4);
        assert_eq!(a * b, Fraction::new(1, 2));

        let a = Fraction::new(1, 2);
        let b = Fraction::new(2, 1);
        assert_eq!(a / b, Fraction::new(1, 4));
    }

    #[test]
    fn test_comparisons() {
        let a = Fraction::new(2, 4);
        let b = Fraction::new(1, 2);
        assert_eq!(a, b);

        let a = Fraction::new(1, 2);
        let b = Fraction::new(3, 4);
        assert!(a < b);
    }

    #[test]
    fn test_special_cases() {
        let zero = Fraction::new(0, 1);
        let a = Fraction::new(3, 4);
        assert_eq!(a + zero, a);
        assert_eq!(a - zero, a);

        assert!(Fraction::new(1, 0).is_infinity());
        assert!(Fraction::new(-1, 0).is_neg_infinity());
        assert!(Fraction::new(0, 0).is_nan());
    }

    #[test]
    fn test_assignment_operations() {
        let mut a = Fraction::new(1, 3);
        a += Fraction::new(1, 6);
        assert_eq!(a, Fraction::new(1, 2));

        let mut b = Fraction::new(3, 4);
        b -= Fraction::new(1, 4);
        assert_eq!(b, Fraction::new(1, 2));

        let mut c = Fraction::new(-7, 6);
        c *= Fraction::new(-8, 7);
        assert_eq!(c, Fraction::new(4, 3));

        let mut d = Fraction::new(-7, 6);
        d /= Fraction::new(-7, 8);
        assert_eq!(d, Fraction::new(4, 3));
    }

    #[test]
    fn test_display_formatting() {
        assert_eq!(format!("{}", Fraction::new(5, 1)), "5");
        
        assert_eq!(format!("{}", Fraction::new(3, 4)), "3/4");
        
        assert_eq!(format!("{}", Fraction::new(-2, 3)), "-2/3");
    }

    #[test]
    fn test_hash_consistency() {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        Fraction::new(2, 4).hash(&mut hasher1);
        Fraction::new(1, 2).hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_edge_cases() {
        let f = Fraction::new(i32::MAX, i32::MAX);
        assert_eq!(f, Fraction::new(1, 1));

        let f = Fraction::new(i32::MIN, i32::MIN);
        assert_eq!(f, Fraction::new(1, 1));
    }

    #[test]
    fn test_sign_handling() {
        assert!(Fraction::new(3, 4).is_positive());
        assert!(Fraction::new(-3, 4).is_negative());
        assert!(Fraction::new(0, 1).is_zero());
    }

    #[test]
    fn test_absolute_value() {
        let f = Fraction::new(-3, 4).abs();
        assert_eq!(f, Fraction::new(3, 4));
    }

    #[test]
    fn test_sqrt() {
        let range = 1.0 / i32::MAX as f64;
        for _ in 0..5000 {
            let m = rand::random_range(0..=i32::MAX);
            let n = rand::random_range(1..=i32::MAX);
            let sqrt = sqrt_fraction(Fraction::new(m, n)).unwrap();
            assert!((f64::from(sqrt) - (m as f64 / n as f64).sqrt()).abs() <= range);
        }
    }

    #[test]
    fn test_document_in_readme() {
        // safe
        let f64_val: f64 = Fraction::new(-5, 4).into();
        assert_eq!(f64_val, -1.25);
        let f64_inf: f64 = f64::from(Fraction::INFINITY);
        assert_eq!(f64_inf, f64::INFINITY);
        // unsafe
        let res_err: Result<i32, _> = Fraction::INFINITY.try_into();
        assert_eq!(res_err, Err(ConversionError::InfiniteConversion));
        let res_ok: Result<i32, _> = i32::try_from(Fraction::new(-3, 2));
        assert_eq!(res_ok, Ok(-1));

        // shrink
        let n = Fraction::new(2147483647, 4);
        let prev = (n + 1) / 2;
        let curr = (n / prev + prev) / 2;
        
        // shrink to 805306375/6 (= 134217729.16666666)
        // not 4611686065672028153/34359738416 (= 134217729.1875)
        assert_eq!(curr, Fraction::new(805306375, 6));
        
        // sqrt
        fn sqrt(n: Fraction) -> Option<Fraction> {
            let mut prev;
            let mut curr;

            if n.is_negative() {
                return None;
            } else if n.is_zero() {
                return Some(Fraction::ZERO);
            } else if (n - 1).is_positive() {
                prev = (n + 1) / 2;
            } else {
                prev = Fraction::from(1);
            }

            curr = (n / prev + prev) / 2;
            loop {
                if curr - prev == Fraction::ZERO {
                    return Some(curr);
                }
                prev = curr;
                curr = (n / prev + prev) / 2;
            }
        }

        assert_eq!(sqrt(Fraction::from(100)).unwrap(), Fraction::from(10));

        let a = Fraction::new(155937625, 24970004);   // 6.244997998398398
        let b = Fraction::new(2103597937, 336845254); // 6.244997998398398
        assert!(a != b);
        assert!(a - b == Fraction::ZERO);
    }
}