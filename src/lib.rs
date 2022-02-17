#![allow(dead_code)]

use std::collections::HashMap;

struct Money {
    amount: f32,
    currency: String,
}

impl Money {
    fn amount(&self) -> f32 {
        self.amount
    }
    fn equals(&self, other: &Self) -> bool {
        self.amount() == other.amount() && self.currency() == other.currency()
    }
    fn times(&self, multiplier: f32) -> Self {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency(),
        }
    }
    fn currency(&self) -> String {
        self.currency.to_owned()
    }
    fn new(value: f32, currency: &str) -> Money {
        Money {
            amount: value,
            currency: currency.to_owned(),
        }
    }
    fn plus(&self, addend: &Money) -> Sum<Money> {
        Sum {
            addend: Money::new(self.amount(), &self.currency()),
            augend: Money::new(addend.amount(), &addend.currency()),
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

impl ToString for Money {
    fn to_string(&self) -> String {
        format!("{} {}", self.amount, self.currency)
    }
}

struct Sum<T> {
    augend: T,
    addend: T,
}

impl<T: Expression> Sum<T> {
    pub fn new(augend: T, addend: T) -> Sum<T> {
        Sum { addend, augend }
    }
}
impl<T: Expression> Expression for Sum<T> {
    fn reduce(&self, to: &str, bank: &Bank) -> Money {
        Money {
            amount: self.addend.reduce(to, bank).amount() + self.augend.reduce(to, bank).amount(),
            currency: to.to_owned(),
        }
    }
}
impl Expression for Money {
    fn reduce(&self, to: &str, bank: &Bank) -> Money {
        let rate = bank.rate(&self.currency(), to);
        Money {
            amount: self.amount() / rate,
            currency: to.to_owned(),
        }
    }
}

struct Bank {
    rates: HashMap<String, f32>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            rates: HashMap::new(),
        }
    }
    pub fn reduce<T: Expression>(&self, source: T, to: &str) -> Money {
        source.reduce(to, &self)
    }

    pub fn add_rate(&mut self, currency: &str, to: &str, rate: f32) {
        if !currency.eq(to) {
            self.rates.insert(format!("{}{}", currency, to), rate);
        }
    }

    pub fn rate(&self, from: &str, to: &str) -> f32 {
        if from.eq(to) {
            1.0
        } else {
            self.rates.get(&format!("{}{}", from, to)).unwrap().clone()
        }
    }
}

trait Expression {
    fn reduce(&self, to: &str, bank: &Bank) -> Money;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Money::new(5.0, "USD");
        assert!(Money::new(10.0, "USD") == five.times(2.0));
        assert!(Money::new(15.0, "USD").equals(&five.times(3.0)));
    }

    #[test]
    fn test_equality() {
        assert!(Money::new(5.0, "USD").equals(&Money::new(5.0, "USD")));
        assert!(!Money::new(5.0, "USD").equals(&Money::new(6.0, "USD")));
        assert!(Money::new(5.0, "CHF").equals(&Money::new(5.0, "CHF")));
    }

    #[test]
    fn test_trait_based_equality() {
        assert!(Money::new(5.0, "USD") == Money::new(5.0, "USD"));
        assert!(Money::new(5.0, "USD") != Money::new(6.0, "USD"));
        assert!(Money::new(5.0, "CHF") != Money::new(5.0, "USD"));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::new(5.0, "USD").currency());
        assert_eq!("CHF", Money::new(5.0, "CHF").currency());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("5 USD", Money::new(5.0, "USD").to_string());
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::new(5.0, "USD");
        let sum = five.plus(&five);
        let bank = Bank::new();
        let reduced = bank.reduce(sum, "USD");
        assert!(reduced == Money::new(10.0, "USD"))
    }

    #[test]
    fn test_plus_returns_sum() {
        let five = Money::new(5.0, "USD");
        let result = five.plus(&five);
        let sum = result;
        assert!(five == sum.augend);
        assert!(five == sum.addend);
    }

    #[test]
    fn test_reduce_sum() {
        let bank = Bank::new();
        let sum = Sum::new(Money::new(3.0, "USD"), Money::new(4.0, "USD"));
        let result = bank.reduce(sum, "USD");
        assert!(Money::new(7.0, "USD") == result);
    }

    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(Money::new(1.0, "USD"), "USD");
        assert!(Money::new(1.0, "USD") == result)
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2.0);
        let result = bank.reduce(Money::new(2.0, "CHF"), "USD");
        assert!(Money::new(1.0, "USD") == result)
    }

    #[test]
    fn test_identity_rate() {
        assert!(1.0 == Bank::new().rate("USD", "USD"))
    }

    #[test]
    fn test_mixed_addition() {
        let five_bucks = Money::new(5.0, "USD");
        let ten_francs = Money::new(10.0, "CHF");
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2.0);
        let result = bank.reduce(five_bucks.plus(&ten_francs), "USD");
        assert!(Money::new(10.0, "USD") == result);
    }
}
