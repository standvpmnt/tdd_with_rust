#![allow(dead_code)]

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
    fn plus(&self, addend: &Money) -> Sum {
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

struct Bank;

impl Bank {
    pub fn reduce<T: Expression>(source: T, to: &str) -> Money {
        source.reduce(to)
    }
}

trait Expression {
    fn reduce(&self, to: &str) -> Money;
}

struct Sum {
    augend: Money,
    addend: Money,
}

impl Sum {
    pub fn new(augend: Money, addend: Money) -> Sum {
        Sum { addend, augend }
    }
}
impl Expression for Sum {
    fn reduce(&self, to: &str) -> Money {
        Money {
            amount: self.addend.amount() + self.augend.amount(),
            currency: to.to_owned(),
        }
    }
}
impl Expression for Money {
    fn reduce(&self, to: &str) -> Money {
        Money {
            amount: self.amount(),
            currency: to.to_owned(),
        }
    }
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
        // let bank = Bank::new();
        let reduced = Bank::reduce(sum, "USD");
        assert!(reduced == Money::new(10.0, "USD"))
    }

    fn test_plus_returns_sum() {
        let five = Money::new(5.0, "USD");
        let result = five.plus(&five);
        let sum: Sum = result;
        assert!(five == sum.augend);
        assert!(five == sum.addend);
    }

    fn test_reduce_sum() {
        let sum = Sum::new(Money::new(3.0, "USD"), Money::new(4.0, "USD"));
        let result = Bank::reduce(sum, "USD");
        assert!(Money::new(7.0, "USD") == result);
    }

    fn test_reduce_money() {
        let result = Bank::reduce(Money::new(1.0, "USD"), "USD");
        assert!(Money::new(1.0, "USD") == result)
    }
}
