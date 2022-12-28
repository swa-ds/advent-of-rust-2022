mod solution {

    const INPUT: &'static str = include_str!("input10.txt");
    
    fn solve(input: &str) -> i32 {
        let instructions = parse(input);
        let mut signal_strength = 0;
        let mut x = 1;
        let mut cycle = 0;

        for instruction in instructions {
            match instruction {
                ("noop", None) => {
                    cycle += 1;
                    calc_signal_strength(&mut signal_strength, cycle, x);
                },
                ("addx", Some(val)) => {
                    cycle += 1;
                    calc_signal_strength(&mut signal_strength, cycle, x);
                    cycle += 1;
                    calc_signal_strength(&mut signal_strength, cycle, x);
                    x += val;
                },
                i => panic!("Invalid instruction {:?}", i)
            }
        }
        signal_strength
    }

    fn calc_signal_strength(signal_strength: &mut i32, cycle: i32, x: i32) {
        let rest = cycle % 20;
        let div = cycle / 20;
        // println!("cycle={} rest={}, div={}", cycle, rest, div);
        if rest == 0 && div % 2 != 0 {
            *signal_strength += cycle * x;
            // println!("cycle={}, x={}, signal strength={}", cycle, x, cycle * x)
        }
    }

    fn parse(input: &str) -> Vec<(&str, Option<i32>)> {
        let mut instructions = vec![];
        for line in input.split("\n") {
            if line == "noop" {
                instructions.push(("noop", None));
            } else {
                let mut split = line.split(" ");
                let instruction = split.next().unwrap();
                let value = split.next().unwrap().parse::<i32>().expect("could not parse");
                instructions.push((instruction, Some(value)));
            }
        }
        instructions
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &'static str = include_str!("input10_test.txt");

        #[test]
        fn test_solve_part_1() {
            assert_eq!(13140, solve(TEST_INPUT));
        }
        
        #[test]
        fn do_solve_part_1() {
            assert_eq!(12980, solve(INPUT));
        }
    }
}