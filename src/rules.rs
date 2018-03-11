use super::*;

pub trait Rule {
    fn is_satisfied(&self, dices: &Dices) -> bool;
    fn reward(&self, dices: &Dices) -> Coins;
    fn consume(&self, dices: Dices) -> Dices;
    fn apply(&self, dices: Dices) -> (Coins, Dices) {
        if !self.is_satisfied(&dices) {
            return (0, dices);
        }
        (self.reward(&dices), self.consume(dices))
    }
}

struct TwoOrLess {
    dice: Dice,
    value: Coins,
}

impl TwoOrLess {
    fn new(dice: Dice) -> Self {
        Self { dice, value: TwoOrLess::value(dice) }
    }

    fn value(dice: Dice) -> Coins {
        use Dice::*;
        match dice {
            Five => 50,
            One => 100,
            _ => 0
        }
    }
}

impl Rule for TwoOrLess {
    fn is_satisfied(&self, dices: &Dices) -> bool {
        dices.occurrence(self.dice) > 0 && dices.occurrence(self.dice) <= 2
    }

    fn reward(&self, dices: &Dices) -> Coins {
        dices.occurrence(self.dice) as Coins * self.value
    }

    fn consume(&self, mut dices: Dices) -> Dices {
        dices.take_all(self.dice);
        dices
    }
}

struct ThreeOrMore {
    dice: Dice,
    value: Coins,
}

impl ThreeOrMore {
    fn new(dice: Dice) -> Self {
        Self { dice, value: ThreeOrMore::value(dice) }
    }

    fn value(dice: Dice) -> Coins {
        use Dice::*;
        match dice {
            Six => 600,
            Five => 500,
            Four => 400,
            Three => 300,
            Two => 200,
            One => 1000
        }
    }
}

impl Rule for ThreeOrMore {
    fn is_satisfied(&self, dices: &Dices) -> bool {
        dices.occurrence(self.dice) >= 3
    }

    fn reward(&self, dices: &Dices) -> Coins {
        (dices.occurrence(self.dice) - 2) as Coins * self.value
    }

    fn consume(&self, mut dices: Dices) -> Dices {
        dices.take_all(self.dice);
        dices
    }
}

struct NoMoreThanOne {
    value: Coins,
    dices: usize
}

impl NoMoreThanOne {
    fn new(value: Coins) -> Self {
        NoMoreThanOne { value, dices: 5 }
    }
}

impl Rule for NoMoreThanOne {
    fn is_satisfied(&self, dices: &Dices) -> bool {
        ALL_FACES.iter().filter(
            |&&d| dices.occurrence(d) == 1)
            .count() == self.dices
    }

    fn reward(&self, _: &Dices) -> Coins {
        self.value
    }

    fn consume(&self, _: Dices) -> Dices {
        Dices::default()
    }
}

pub type Rules = Vec<Box<Rule>>;

pub fn three_or_more(dice: &Dice) -> Box<Rule> {
    Box::new(ThreeOrMore::new(dice.clone())) as Box<Rule>
}

pub fn two_or_less(dice: &Dice) -> Box<Rule> {
    Box::new(TwoOrLess::new(dice.clone())) as Box<Rule>
}

pub fn no_more_than_one(value: Coins) -> Box<Rule> {
    Box::new(NoMoreThanOne::new(value)) as Box<Rule>
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest_parametrize;

    trait IntoDices {
        fn into_dices(self) -> Dices;
    }

    impl<'a> IntoDices for &'a str {
        fn into_dices(self) -> Dices {
            self.chars().map(Dice::from).into()
        }
    }

    #[rstest_parametrize(
    dices, reward, remain,
    case("22554", 100, "224"),
    case("35446", 50, "3446"),
    case("32446", 0, "32446"),
    )]
    fn dice_5_test_two_or_less(dices: &str, reward: Coins, remain: &str) {
        let dices = dices.into_dices();
        let remain = remain.into_dices();

        assert_eq!((reward, remain), TwoOrLess::new(Dice::Five).apply(dices))
    }

    #[rstest_parametrize(
    dices, reward, remain,
    case("23456", 200, ""),
    case("12345", 200, ""),
    case("62154", 200, ""),
    case("66154", 0, "66154"),
    case("6154", 0, "6154"),
    )]
    fn five_different_dices(dices: &str, reward: Coins, remain: &str) {
        let dices = dices.into_dices();
        let remain = remain.into_dices();

        assert_eq!((reward, remain), NoMoreThanOne::new(200).apply(dices))
    }
}
