#[allow(dead_code)]
mod solution {
    use std::collections::HashSet;

    const INPUT: &'static str = include_str!("input06.txt");

    fn solve_part_1(input: &str) -> Option<usize> {
        solve(input, 4)
    }

    fn solve_part_2(input: &str) -> Option<usize> {
        solve(input, 14)
    }

    fn solve(input: &str, distinct_chars: usize) -> Option<usize> {
        let mut last_chars: HashSet<char> = HashSet::new();
        for i in distinct_chars..input.len() {
            let _ = &input[i-distinct_chars..i].chars().for_each(|c| { last_chars.insert(c); () });
            // println!("{} {:?}", i, last_chars);
            if last_chars.len() == distinct_chars {
                return Some(i);
            }
            last_chars.clear();
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_part_1() {
            assert_eq!(Some(7), solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
            assert_eq!(Some(5), solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
            assert_eq!(Some(6), solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"));
            assert_eq!(Some(10), solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
            assert_eq!(Some(11), solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
        }

        #[test]
        fn test_solve_part_2() {
            assert_eq!(Some(19), solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
            assert_eq!(Some(23), solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
            assert_eq!(Some(23), solve_part_2("nppdvjthqldpwncqszvftbrmjlhg"));
            assert_eq!(Some(29), solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
            assert_eq!(Some(26), solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(Some(1262), solve_part_1(INPUT));
        }

        #[test]
        fn do_solve_part_2() {
            assert_eq!(Some(3444), solve_part_2(INPUT));
        }

    }

}
