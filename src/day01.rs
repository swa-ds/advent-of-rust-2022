mod solution {

    const INPUT: &'static str = include_str!("input01.txt");

    fn solve_part_1(input: &str) -> usize {
        let calories_per_elf = calories_by_elf(input);
        calories_per_elf[0]
    }

    fn solve_part_2(input: &str) -> usize {
        let calories_per_elf = calories_by_elf(input);
        calories_per_elf[0..3].iter().sum()
    }

    fn calories_by_elf(input: &str) -> Vec<usize> {
        let lines = input.split("\n");
        let mut calories_per_elf: Vec<usize> = vec![];
        let mut sum = 0;

        lines.for_each(|line| {
            if line.is_empty() {
                calories_per_elf.push(sum);
                sum = 0;
            } else {
                sum += line.parse::<usize>().unwrap();
            }
        });
        calories_per_elf.push(sum);
        calories_per_elf.sort_by(|a, b| b.cmp(a));
        calories_per_elf
    }

    #[cfg(test)]
    mod tests {
        const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        use super::*;

        #[test]
        fn test_solve_part_1() {
            assert_eq!(24000, solve_part_1(TEST_INPUT));
        }

        #[test]
        fn test_solve_part2() {
            assert_eq!(45000, solve_part_2(TEST_INPUT));
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(71300, solve_part_1(INPUT))
        }

        #[test]
        fn do_solve_part_2() {
            assert_eq!(209691, solve_part_2(INPUT))
        }
    }
}
