mod solution {
    use std::ops::RangeInclusive;
    use crate::day04::solution::Solution::*;

    const INPUT: &'static str = include_str!("input04.txt");

    enum Solution {
        PartOne,
        PartTwo,
    }

    fn solve(input: &str, part: Solution) -> usize {
        let mut count = 0;
        let lines = input.split("\n").into_iter();
        for line in lines {
            let mut sp = line.split(",");
            let r1 = to_range(sp.next());
            let r2 = to_range(sp.next());
            match part {
                PartOne => if contains(r1, r2) { count += 1 },
                PartTwo => if overlaps(r1, r2) { count += 1 },
            }            
        }
        count
    }

    fn overlaps(r1: RangeInclusive<usize>, r2: RangeInclusive<usize>) -> bool {
        r1.start() <= r2.end() && r2.start() <= r1.end()
    }

    fn contains(r1: RangeInclusive<usize>, r2: RangeInclusive<usize>) -> bool {
        (r1.start() >= r2.start() && r1.end() <= r2.end()) ||
            (r1.start() <= r2.start() && r1.end() >= r2.end())
    }


    fn to_range(str: Option<&str>) -> RangeInclusive<usize> {
        // println!("{:?}", str);
        let mut split = str.unwrap().split("-");
        let start = split.next().unwrap().parse::<usize>().unwrap();
        let end = split.next().unwrap().parse::<usize>().unwrap();
        start..=end
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        #[test]
        fn test_solve_part_1() {
            assert_eq!(2, solve(TEST_INPUT, PartOne));
        }

        #[test]
        fn test_solve_part_2() {
            assert_eq!(4, solve(TEST_INPUT, PartTwo));
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(431, solve(INPUT, PartOne));
        }

        #[test]
        fn do_solve_part_2() {
            assert_eq!(823, solve(INPUT, PartTwo));
        }
    }
}
