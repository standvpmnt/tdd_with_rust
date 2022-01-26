struct Dollar {
    amount: f32,
}

impl Dollar {
    pub fn new(value: f32) -> Dollar {
        Dollar { amount: value }
    }

    pub fn times(&mut self, multiplier: f32) {
        self.amount *= multiplier;
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
        let mut five = Dollar::new(5.0);
        five.times(2.0);
        assert_eq!(10.0, five.amount)
    }
}
