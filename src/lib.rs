#![allow(dead_code)]

struct Dollar {
    amount: f32,
    currency: String,
}

trait Money {
    fn amount(&self) -> f32;
    fn equals(&self, other: &Self) -> bool {
        self.amount() == other.amount()
    }
    fn times(&self, multiplier: f32) -> Self;
    fn currency(&self) -> String;
}

impl Money for Dollar {
    fn amount(&self) -> f32 {
        self.amount
    }
    fn times(&self, multiplier: f32) -> Dollar {
        Dollar {
            amount: &self.amount * multiplier,
            currency: "USD".to_owned(),
        }
    }
    fn currency(&self) -> String {
        "USD".to_owned()
    }
}

impl Dollar {
    pub fn new(value: f32) -> Dollar {
        Dollar {
            amount: value,
            currency: "USD".to_owned(),
        }
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

struct Franc {
    amount: f32,
    currency: String,
}

impl Money for Franc {
    fn amount(&self) -> f32 {
        self.amount
    }
    fn times(&self, multiplier: f32) -> Franc {
        Franc {
            amount: &self.amount * multiplier,
            currency: "CHF".to_owned(),
        }
    }
    fn currency(&self) -> String {
        "CHF".to_owned()
    }
}

impl Franc {
    pub fn new(value: f32) -> Franc {
        Franc {
            amount: value,
            currency: "CHF".to_owned(),
        }
    }
}

impl PartialEq for Franc {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5.0);
        assert!(Dollar::new(10.0) == five.times(2.0));
        assert!(Dollar::new(15.0) == five.times(3.0));
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
        // assert!(Dollar::new(5.0) != Franc::new(5.0));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5.0);
        assert!(Franc::new(10.0) == five.times(2.0));
        assert!(Franc::new(15.0) == five.times(3.0));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Dollar::new(5.0).currency());
        assert_eq!("CHF", Franc::new(5.0).currency());
    }
}
