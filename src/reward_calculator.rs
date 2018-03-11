use rules::*;
use super ::*;

pub fn calculator() -> RewardCalculator {
    use Dice::*;
    let mut rules: Rules = ALL_FACES.iter()
        .map(three_or_more)
        .collect();
    rules.extend([One, Five].iter().map(two_or_less));
    rules.push(no_more_than_one(200));
    RewardCalculator::new(rules)
}

pub trait Calculator {
    fn calculate(&self, dices: Dices) -> Coins;
}

pub struct RewardCalculator {
    rules: Rules
}

impl Calculator for RewardCalculator {
    fn calculate(&self, mut dices: Dices) -> Coins {
        let mut amount = 0;
        while let Some(r) = self.most_rewarded_rule(&dices) {
            let (reward, d) = r.apply(dices);
            amount += reward;
            dices = d;
        }
        amount
    }
}

impl RewardCalculator {
    fn new(rules: Rules) -> Self {
        Self { rules }
    }

    fn most_rewarded_rule(&self, dices: &Dices) -> Option<&Box<Rule>> {
        self.rules.iter()
            .filter(|r| r.is_satisfied(dices))
            .max_by_key(|r| r.reward(dices))
    }
}
