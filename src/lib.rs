#![allow(dead_code)]

struct Dollar;

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
}

impl Dollar {
    pub fn new(value: f32) -> Money {
        Money {
            amount: value,
            currency: "USD".to_owned(),
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

struct Franc;

impl Franc {
    pub fn new(value: f32) -> Money {
        Money {
            amount: value,
            currency: "CHF".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5.0);
        assert!(Dollar::new(10.0) == five.times(2.0));
        assert!(Dollar::new(15.0).equals(&five.times(3.0)));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5.0).equals(&Dollar::new(5.0)));
        assert!(!Dollar::new(5.0).equals(&Dollar::new(6.0)));
        assert!(Franc::new(5.0).equals(&Franc::new(5.0)));
        assert!(!Franc::new(5.0).equals(&Franc::new(6.0)));
        // assert!(
        //     !Franc::new(5.0).equals(&Dollar::new(5.0)),
        //     "Dollar and franc are not equal"
        // );
    }

    #[test]
    fn test_trait_based_equality() {
        assert!(Dollar::new(5.0) == Dollar::new(5.0));
        assert!(Dollar::new(5.0) != Dollar::new(6.0));
        assert!(Dollar::new(5.0) != Franc::new(5.0));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5.0);
        assert!(Franc::new(10.0).equals(&five.times(2.0)));
        assert!(Franc::new(15.0).equals(&five.times(3.0)));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Dollar::new(5.0).currency());
        assert_eq!("CHF", Franc::new(5.0).currency());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("5 USD", Dollar::new(5.0).to_string());
    }
}
