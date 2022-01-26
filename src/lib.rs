struct Dollar {
    amount: f32,
}

impl Dollar {
    pub fn new(value: f32) -> Dollar {
        Dollar { amount: value }
    }

    pub fn times(&self, multiplier: f32) -> Dollar {
        Dollar {
            amount: self.amount * multiplier,
        }
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
        let product = five.times(2.0);
        assert_eq!(10.0, product.amount);
        let product = five.times(3.0);
        assert_eq!(15.0, product.amount);
    }
}
