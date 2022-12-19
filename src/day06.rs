#[allow(dead_code)]
mod solution {
    use std::collections::HashSet;

    const INPUT: &'static str = include_str!("input06.txt");

    fn solve(input: &str) -> Option<usize> {
        let mut last_chars: HashSet<char> = HashSet::new();
        for i in 4..input.len() {
            let _ = &input[i-4..i].chars().for_each(|c| { last_chars.insert(c); () });
            // println!("{} {:?}", i, last_chars);
            if last_chars.len() == 4 {
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
            assert_eq!(Some(7), solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
            assert_eq!(Some(5), solve("bvwbjplbgvbhsrlpgdmjqwftvncz"));
            assert_eq!(Some(6), solve("nppdvjthqldpwncqszvftbrmjlhg"));
            assert_eq!(Some(10), solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
            assert_eq!(Some(11), solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(Some(1262), solve(INPUT));
        }

    }

}
