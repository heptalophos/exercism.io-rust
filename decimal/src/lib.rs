use std::{iter, cmp::{Ordering, max, min}, ops::{Add, Sub, Mul}};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    neg: bool,
    digits: Vec<u8>,
    power: isize,
}

impl Decimal {
    pub fn new() -> Decimal {
        Decimal {neg: false, digits: Vec::new(), power: 0}
    }

    pub fn try_from(num: &str) -> Option<Decimal> {
        let mut decimal = Decimal::new();
        let mut start = 0;
        if num.starts_with("-") {
            start = 1;
            decimal.neg = true;
        } else if num.starts_with("+") {
            start = 1;
        }
        for (i, d) in num[start..].chars().rev().enumerate() {
            if d == '.' {
                decimal.power = -(i as isize)
            } else if let Some(digit) = d.to_digit(10) {
                decimal.digits.push(digit as u8) 
            } else {
                return None
            }
        };
        Some(decimal.reduced_form())
    }

    fn zip(this: &Decimal, that: &Decimal) -> Vec<(u8, u8)> {
        let mut zip = vec![];
        for i in 0..max(this.digits.len(), that.digits.len()) {
            zip.push((*this.digits.get(i).unwrap_or(&0), 
                      *that.digits.get(i).unwrap_or(&0)));
        }
        zip
    }

    fn reduce(this: &Decimal, that: &Decimal) -> Vec<(u8, u8)> {
        let mut reduced = Decimal::new();
        if this.power < that.power {
            return Decimal::reduce(that, this).iter()
                   .map(|&(x, y)| (y, x)).collect::<Vec<(u8, u8)>>()
        }
        reduced.digits = iter::repeat(0)
                         .take((this.power - that.power) as usize)
                         .chain(this.digits.clone())
                         .collect::<Vec<u8>>();
        reduced.power = that.power;
        Decimal::zip(&reduced, that)
    }

    fn flip_neg(&self) -> Decimal {
        let mut flipped = self.clone();
        flipped.neg = !self.neg;
        flipped
    }

    fn reduced_form(&self) -> Decimal {
        let mut normal = self.clone();
        while let Some(&d) = normal.digits.last() {
            if d != 0 { break; }
            normal.digits.pop();
        }
        while let Some(&d) = normal.digits.first() {
            if d != 0 { break; }
            normal.power += 1;
            normal.digits.remove(0); 
        }
        if normal.digits == [] {
            normal = Decimal::new();
            normal.digits.push(0);
        }
        normal
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        Some(Ordering::Equal) == 
        self.reduced_form()
            .partial_cmp(&other.reduced_form()) 
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        if self.neg && !other.neg { return Some(Ordering::Less) }
        else if !self.neg && other.neg { return Some(Ordering::Greater) }
        for &(x, y) in Decimal::reduce(self, other).iter().rev() {
            if x != y { 
                if !self.neg { return x.partial_cmp(&y) }
                else { return y.partial_cmp(&x) }
            }
        }
        Some(Ordering::Equal)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        let (x, y) = (self.reduced_form(), other.reduced_form());
        
        let mut sum = Decimal::new();
        sum.power = min(x.power, y.power);
        
        if x.neg && y.neg { sum.neg = true; }
        else if x.neg { return y.sub(x.flip_neg()) }
        else if y.neg { return x.sub(y.flip_neg()) }
        
        let mut carry = 0;
        for (a, b) in Decimal::reduce(&x, &y) {
            sum.digits.push((a + b + carry) % 10);
            carry = (a + b + carry) / 10;
        }
        if carry != 0 { sum.digits.push(carry) }
        
        sum.reduced_form()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        if other.neg { return self.add(other.flip_neg()) }
        else if self < other { return other.sub(self).flip_neg() }
        
        let (x, y) = (self.reduced_form(), other.reduced_form());
        
        let mut diff = Decimal::new();
        diff.power = min(x.power, y.power);
        
        let mut carry = 0;
        for (a, b) in Decimal::reduce(&x, &y) {
            if a >= b + carry {
                diff.digits.push(a - b - carry);
                carry = 0;
            } else {
                diff.digits.push(a + 10 - b - carry);
                carry = 1;
            }
        }
        
        diff.reduced_form()
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        let (x, y) = (self.reduced_form(), other.reduced_form());
        
        let mut prod = Decimal::new();
        prod.power = x.power + y.power;

        for (i, a) in x.digits.iter().enumerate() {
            let mut row = Decimal::new();
            row.power = i as isize + prod.power;
            let mut carry = 0;
            for b in &y.digits {
                row.digits.push((a * b + carry) % 10);
                carry = (a * b + carry) / 10; 
            }
            if carry > 0 { row.digits.push(carry); }
            prod = prod.add(row);
        }
        prod.neg = x.neg != y.neg;

        prod.reduced_form()
    }
}
