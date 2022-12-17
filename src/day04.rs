mod solution {
    use std::ops::RangeInclusive;

    const INPUT: &'static str = include_str!("input04.txt");

    fn solve(input: &str) -> usize {
        let mut count = 0;
        let lines = input.split("\n").into_iter();
        for line in lines {
            let mut sp = line.split(",");
            let r1 = to_range(sp.next());
            let r2 = to_range(sp.next());
            if r1.start() >= r2.start() && r1.end() <= r2.end() {
                count += 1;
            } else if r1.start() <= r2.start() && r1.end() >= r2.end() {
                count += 1;
            }
        }
        count
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
            assert_eq!(2, solve(TEST_INPUT));
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(431, solve(INPUT));
        }

    }

}
