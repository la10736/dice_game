#![feature(proc_macro, option_filter)]
extern crate rstest;

mod rules;
mod reward_calculator;

pub type Coins = u32;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Dice {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub static ALL_FACES: [Dice; 6] =
    [Dice::One, Dice::Two, Dice::Three, Dice::Four, Dice::Five, Dice::Six];

type DiceOccurrences = std::collections::HashMap<Dice, usize>;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Dices {
    occurrences: DiceOccurrences
}

impl<I: IntoIterator<Item=Dice>> From<I> for Dices {
    fn from(iter: I) -> Self {
        let mut occurrences = DiceOccurrences::new();
        iter.into_iter().for_each(
            |d| *occurrences.entry(d)
                .or_insert(0) += 1
        );
        Dices { occurrences }
    }
}

impl Dices {
    fn occurrence(&self, dice: Dice) -> usize {
        *self.occurrences.get(&dice).unwrap_or(&0)
    }

    fn take_all(&mut self, dice: Dice) -> usize {
        self.occurrences
            .remove(&dice)
            .unwrap_or(0)
    }
}

use reward_calculator::*;

struct Throw {
    dices: Dices
}

impl Throw {
    fn reward(&self) -> Coins {
        calculator().calculate(self.dices.clone())
    }
}

#[cfg(test)]
impl From<char> for Dice {
    fn from(c: char) -> Self {
        use Dice::*;
        match c {
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            _ => panic!("Invalid dice")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest_parametrize;

    impl<S: AsRef<str>> From<S> for Throw {
        fn from(s: S) -> Self {
            let dices = s
                .as_ref()
                .chars()
                .map(Dice::from)
                .into();

            Throw { dices }
        }
    }

    #[rstest_parametrize(
    dices, reward,
    case("62233", 0),
    case("62663", 600),
    case("66626", 1200),
    case("66666", 1800),
    case("63555", 500),
    case("55455", 1000),
    case("55555", 1500),
    case("64442", 400),
    case("44644", 800),
    case("44444", 1200),
    case("33342", 300),
    case("33433", 600),
    case("33333", 900),
    case("33222", 200),
    case("32222", 400),
    case("22222", 600),
    case("11661", 1000),
    case("31111", 2000),
    case("11111", 3000),
    case("11446", 200),
    case("31446", 100),
    case("22554", 100),
    case("35446", 50),
    case("14445", 550),
    case("11222", 400),
    case("12345", 200),
    case("23456", 200),
    case("23416", 200),
    )]
    fn throw_should_reward(dices: &str, reward: Coins) {
        let throw: Throw = dices.into();

        assert_eq!(reward, throw.reward())
    }
}
