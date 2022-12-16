#[allow(dead_code,unused_variables)]
mod solution {
    use std::collections::HashMap;
    use crate::day02::solution::Item::*;
    use crate::day02::solution::Part::*;
    use crate::day02::solution::Outcome::*;

    const INPUT: &'static str = include_str!("input02.txt");

    enum Part {
        PartOne,
        PartTwo,
    }

    #[derive(Copy, Clone, Debug)]
    enum Item {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Copy, Clone, Debug)]
    enum Outcome {
        Win,
        Lose,
        Draw,
    }

    lazy_static! {
        // maps char (bytes) to Item
        static ref ITEMS: HashMap<u8, Item> =[
            (65, Rock),
            (66, Paper),
            (67, Scissors),
            (88, Rock),
            (89, Paper),
            (90, Scissors)]
        .iter().cloned().collect();
    }

    lazy_static! {
        static ref OUTCOMES: HashMap<u8, Outcome> =[
            (88, Lose),
            (89, Draw),
            (90, Win)]
        .iter().cloned().collect();
    }

    fn solve(input: &str, part: Part) -> usize {
        let mut score = 0;
        let rounds = input.split("\n");
        for (i, round) in rounds.filter(|r| !r.is_empty()).into_iter().enumerate() {
            let play : &[u8] = round.as_bytes();
            let opponent = ITEMS.get(&play[0]).unwrap();
            let me = ITEMS.get(&play[2]).unwrap();
            score += match &part {
                PartOne => play_round_part_1(me, opponent),
                PartTwo => play_round_part_2(opponent, &play[2]),
            };
        }
        score
    }

    fn play_round_part_1(me: &Item, opponent: &Item) -> usize {
        item_score(me) + outcome_score(me, *opponent)
    }

    fn play_round_part_2(opponent: &Item, outcome_indicator: &u8) -> usize {
        let outcome = OUTCOMES.get(outcome_indicator).unwrap();
        let me= match (outcome, opponent) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            _ => *opponent
        };
        item_score(&me) + outcome_score(&me, *opponent)
    }

    fn outcome_score(me: &Item, opponent: Item) -> usize {
        match (me, opponent) {
            // "I win" scenarios
            (Paper, Rock) => 6,
            (Scissors, Paper) => 6,
            (Rock, Scissors) => 6,
            // "I lose" scenarios
            (Rock, Paper) => 0,
            (Paper, Scissors) => 0,
            (Scissors, Rock) => 0,
            // draw scenario (both chose the same item)
            _ => 3,
        }
    }

    fn item_score(me: &Item) -> usize {
        match me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = "A Y
B X
C Z";

        #[test]
        fn test_solve_part_1() {
            assert_eq!(15, solve(TEST_INPUT, PartOne));
        }

        #[test]
        fn test_solve_part_2() {
            assert_eq!(12, solve(TEST_INPUT, PartTwo));
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(10816, solve(INPUT, PartOne));
        }

        #[test]
        fn do_solve_part_2() {
            assert_eq!(11657, solve(INPUT, PartTwo));
        }

    }
}
