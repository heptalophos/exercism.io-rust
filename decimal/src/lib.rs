use std::{iter, cmp::{Ordering, max, min}, ops::{Add, Sub, Mul}};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    sign: i8,
    point: isize,
    digits: Vec<u8>,
}

impl Decimal {
    pub fn new() -> Decimal {
        Decimal { sign: 1, point: 0, digits: Vec::new() }
    }

    pub fn try_from(num: &str) -> Option<Decimal> {
        let mut decimal = Decimal::new();
        let mut start = 0;
        if num.starts_with("-") {
            start = 1;
            decimal.sign = -1;
        } else if num.starts_with("+") {
            start = 1;
        }
        for (i, d) in num[start..].chars().rev().enumerate() {
            if d == '.' {
                decimal.point = -(i as isize)
            } else if let Some(digit) = d.to_digit(10) {
                decimal.digits.push(digit as u8) 
            } else {
                return None
            }
        };
        Some( decimal )
    }

    fn associate(this: &Decimal, that: &Decimal) -> Vec<(u8, u8)> {
        let mut redc = Decimal::new();
        if this.point < that.point {
            return Decimal::associate(that, this).iter()
                           .map(|&(x, y)| (y, x))
                           .collect::<Vec<(u8, u8)>>()
        }
        redc.digits = iter::repeat(0)
                     .take((this.point - that.point) as usize)
                     .chain(this.digits.clone()).collect::<Vec<u8>>();
        redc.point = that.point;
        let mut zipped = vec![];
        for i in 0..max(redc.digits.len(), that.digits.len()) {
            zipped.push((*redc.digits.get(i).unwrap_or(&0), 
                         *that.digits.get(i).unwrap_or(&0)));
        }
        zipped
    }

    fn change_sign(&self) -> Decimal {
        let mut flipped = self.clone();
        flipped.sign = -self.sign;
        flipped
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        if self.sign * other.sign < 0 {
            if self.sign > 0 { 
                return Some(Ordering::Greater) 
            } else { 
                return Some(Ordering::Less) 
            }
        }
        for &(x, y) in Decimal::associate(self, other).iter().rev() {
            if x != y { 
                if self.sign > 0 { 
                    return x.partial_cmp(&y) 
                } else { 
                    return y.partial_cmp(&x) 
                }
            }
        }
        Some(Ordering::Equal)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        self.partial_cmp(&other) == Some(Ordering::Equal)
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        let mut sum = Decimal::new();
        sum.point = min(self.point, other.point);
        if self.sign * other.sign > 0 { 
            sum.sign = self.sign; 
        } else 
        if self.sign < 0 { 
            return other.sub(self.change_sign()) 
        } else { 
            return self.sub(other.change_sign()) 
        }
        let mut carry = 0;
        for (a, b) in Decimal::associate(&self, &other) {
            sum.digits.push((a + b + carry) % 10);
            carry = (a + b + carry) / 10;
        }
        if carry != 0 { sum.digits.push(carry) }
        sum
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        if other.sign < 0 { 
            return self.add(other.change_sign()) 
        } else 
        if self < other { 
            return other.sub(self).change_sign() 
        }
        let mut diff = Decimal::new();
        diff.point = min(self.point, other.point);
        let mut carry = 0;
        for (a, b) in Decimal::associate(&self, &other) {
            if a >= b + carry {
                diff.digits.push(a - b - carry);
                carry = 0;
            } else {
                diff.digits.push(a + 10 - b - carry);
                carry = 1;
            }
        }
        diff
    }
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        let mut prod = Decimal::new();
        prod.point = self.point + other.point;
        for (i, a) in self.digits.iter().enumerate() {
            let mut row = Decimal::new();
            row.point = prod.point + i as isize;
            let mut carry = 0;
            for b in &other.digits {
                row.digits.push((a * b + carry) % 10);
                carry = (a * b + carry) / 10; 
            }
            if carry > 0 { row.digits.push(carry); }
            prod = prod.add(row);
        }
        prod.sign = -self.sign & other.sign;
        prod
    }
}
