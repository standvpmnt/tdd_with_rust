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
        let temp = &self.currency.to_owned();
        temp.to_owned()
    }
    fn new(value: f32, currency: &str) -> Money {
        Money {
            amount: value,
            currency: currency.to_owned(),
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
}
