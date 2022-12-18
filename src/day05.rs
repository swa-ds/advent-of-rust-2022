#[allow(dead_code)]
mod solution {
    use regex::Regex;
    use crate::Solution;
    use crate::Solution::*;

    const INPUT: &'static str = include_str!("input05.txt");

    fn solve(input: &str, part: Solution) -> String {
        let re_stacks = Regex::new(r"(\[[A-Z]]| {3,4})").unwrap();
        let re_move = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

        let line_len = input.find('\n').unwrap();
        let stacks_count = (line_len + 1) / 4;
        // println!("Number of stacks: {}", stacks_count);
        let mut stacks: Vec<Vec<char>> = vec![];
        for _ in 0..stacks_count {
            stacks.push(vec![]);
        }

        let lines = input.split("\n").into_iter();
        for line in lines {            
            if re_stacks.is_match(line) {
                if line.starts_with(" 1   2   3 ") || line.is_empty() { continue; };
                for (i, cap) in re_stacks.captures_iter(line).enumerate() {
                    let crt = cap[0][1..2].chars().next().unwrap();
                    if crt != ' ' { stacks[i].insert(0, crt) };
                }
            }
            if re_move.is_match(line) {
                let cap = re_move.captures(line).unwrap();
                let count = cap[1].parse::<usize>().expect(&cap[1]);
                let from = cap[2].parse::<usize>().expect(&cap[2]);
                let to = cap[3].parse::<usize>().expect(&cap[3]);
                match part {
                    PartOne => move_part_1(&mut stacks, count, from, to),
                    PartTwo => move_part_2(&mut stacks, count, from, to),
                }                
            }                            
        }
        let mut result = String::from("");
        for i in 0..stacks_count {
            let c = *stacks[i].last().unwrap();
            result.push(c);
        }
        result
    }

    fn move_part_1(stacks: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) -> () {
        for _ in 0..count {
            let crt = stacks[from-1].pop().unwrap();
            stacks[to-1].push(crt);
        }
    }

    fn move_part_2(stacks: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) -> () {
        let mut temp_stack = vec![];
        for _ in 0..count {
            let crt = stacks[from-1].pop().unwrap();
            temp_stack.push(crt);
        }
        for _ in 0..count {
            stacks[to-1].push(temp_stack.pop().unwrap());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        #[test]
        fn test_solve_part_1() {
            assert_eq!("CMZ", solve(TEST_INPUT, PartOne))
        }
        
        #[test]
        fn test_solve_part_2() {
            assert_eq!("MCD", solve(TEST_INPUT, PartTwo))
        }
        
        #[test]
        fn do_solve_part_1() {
            assert_eq!("RNZLFZSJH", solve(INPUT, PartOne))
        }

        #[test]
        fn do_solve_part_2() {
            assert_eq!("CNSFCGJSM", solve(INPUT, PartTwo))
        }

    }
}
