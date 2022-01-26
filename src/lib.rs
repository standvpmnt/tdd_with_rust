struct Dollar {
    amount: f32,
}

impl Dollar {
    pub fn new(value: f32) -> Dollar {
        Dollar { amount: value }
    }

    pub fn times(&self, multiplier: f32) -> Dollar {
        Dollar {
            amount: &self.amount * multiplier,
        }
    }

    pub fn equals(&self, other: &Dollar) -> bool {
        self.amount == other.amount
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

struct Franc {
    amount: f32,
}

impl Franc {
    pub fn new(value: f32) -> Franc {
        Franc { amount: value }
    }

    pub fn times(&self, multiplier: f32) -> Franc {
        Franc {
            amount: &self.amount * multiplier,
        }
    }

    pub fn equals(&self, other: &Dollar) -> bool {
        self.amount == other.amount
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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
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
    }

    #[test]
    fn test_trait_based_equality() {
        assert!(Dollar::new(5.0) == Dollar::new(5.0));
        assert!(Dollar::new(5.0) != Dollar::new(6.0));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5.0);
        assert!(Franc::new(10.0) == five.times(2.0));
        assert!(Franc::new(15.0) == five.times(3.0));
    }
}
