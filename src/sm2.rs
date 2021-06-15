#[derive(PartialEq, Debug)]
pub struct SmValue {
    pub repetitions: i64,
    pub interval: i64,
    pub ease_factor: f64,
}

impl SmValue {
    pub fn inital() -> Self {
        SmValue {
            repetitions: 0,
            interval: 0,
            ease_factor: 2.5,
        }
    }
}

pub fn calc(quality: u32, prev: SmValue) -> SmValue {
    let (interval, repetitions, mut ease_factor) = if quality >= 3 {
        let interval = match prev.repetitions {
            0 => 1,
            1 => 6,
            int => (int as f64 * prev.ease_factor) as i64,
        };
        let repetitions = prev.repetitions + 1;
        let ease_factor =
            prev.ease_factor + (0.1 - (5 - quality) as f64 * (0.08 + (5 - quality) as f64 * 0.02));
        (interval, repetitions, ease_factor)
    } else {
        (1, 0, prev.ease_factor)
    };

    if ease_factor < 1.3 {
        ease_factor = 1.3;
    }

    SmValue {
        interval,
        repetitions,
        ease_factor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_review_works() {
        let quality = 5;
        let prev = SmValue::inital();
        let resp = calc(quality, prev);
        let expected = SmValue {
            repetitions: 1,
            interval: 1,
            ease_factor: 2.6,
        };
        assert_eq!(resp, expected);
    }

    #[test]
    fn second_review_works() {
        let quality = 5;
        let prev = SmValue {
            repetitions: 1,
            interval: 1,
            ease_factor: 2.6,
        };
        let resp = calc(quality, prev);
        let expected = SmValue {
            repetitions: 2,
            interval: 6,
            ease_factor: 2.7,
        };
        assert_eq!(resp, expected);
    }

    #[test]
    fn third_review_works() {
        let quality = 5;
        let prev = SmValue {
            repetitions: 2,
            interval: 5,
            ease_factor: 2.5,
        };
        let resp = calc(quality, prev);
        let expected = SmValue {
            repetitions: 3,
            interval: 5,
            ease_factor: 2.6,
        };
        assert_eq!(resp, expected);
    }
}
