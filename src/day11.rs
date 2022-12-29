mod solution {
    use std::cell::RefCell;
    use regex::Regex;

    const INPUT: &'static str = include_str!("input11.txt");


    const REGEX: &str = r"Monkey (\d):
  Starting items: (.*)
  Operation: new = old ([+*]) (\d+|old)
  Test: divisible by (\d+)
    If true: throw to monkey (\d)
    If false: throw to monkey (\d)";

    #[derive(Debug)]
    struct Monkey<'a> {
        id: usize,
        items: RefCell<Vec<usize>>,
        inspect_operator: &'a str,
        inspect_op_arg: &'a str,
        divisible_by: usize,
        true_monkey_id: usize,
        false_monkey_id: usize,
        inspected_items: RefCell<usize>,
    }
    
    fn solve_part_1(input: &str) -> usize {
        let mut monkeys = parse(input);

        for _ in 0..20 {
            play_round(&mut monkeys);
        }

        // for m in &monkeys {
        //     println!("{:?}", m);
        // }

        let mut monkey_business : Vec<usize> = monkeys.iter()
            .map(|m| m.inspected_items.take())
            .collect();
        
        monkey_business.sort_by(|a, b| b.cmp(a));
        // println!("{:?}", monkey_business);
        
        monkey_business.into_iter()
            .take(2)
            .fold(1, |a, b| a * b)
    }

    fn play_round(monkeys: &mut Vec<Monkey>) {
        for monkey in monkeys.iter() {            
            while !monkey.items.borrow().is_empty() {
                let it = monkey.items.borrow_mut().remove(0);
                let mut it_new = operate(it, monkey.inspect_operator, monkey.inspect_op_arg);
                it_new = it_new / 3;
                let to_throw = match it_new % monkey.divisible_by == 0 {
                    true => &monkeys[monkey.true_monkey_id],
                    false => &monkeys[monkey.false_monkey_id],
                };
                to_throw.items.borrow_mut().push(it_new);
                monkey.inspected_items.replace_with(|v| *v + 1 as usize);
            }
        }                
    }

    fn operate(it: usize, operator: &str, arg_str: &str) -> usize {
        let arg = match arg_str {
            "old" => it,
            _ => arg_str.parse().expect("should be able to parse argument"),
        };
        match operator {
            "+" => it + arg,
            "*" => it * arg,
            _ => panic!("invalid operator")
        }
    }

    fn parse(input: &str) -> Vec<Monkey> {
        let regex = Regex::new(REGEX).expect("Invalid regex!");

        let mut monkeys = vec![];

        for split_monkey in input.split("\n\n") {
            let caps = regex.captures(split_monkey).unwrap();

            let mut i = 1;
            while caps.get(i).is_some() {
                let monkey = Monkey {
                    id: caps.get(i).unwrap().as_str().parse::<usize>().unwrap(),
                    items: parse_items(caps.get(i + 1).unwrap().as_str()),
                    inspect_operator: caps.get(i + 2).unwrap().as_str(),
                    inspect_op_arg: caps.get(i + 3).unwrap().as_str(),
                    divisible_by: caps.get(i + 4).unwrap().as_str().parse::<usize>().unwrap(),
                    true_monkey_id: caps.get(i + 5).unwrap().as_str().parse::<usize>().unwrap(),
                    false_monkey_id: caps.get(i + 6).unwrap().as_str().parse::<usize>().unwrap(),
                    inspected_items: RefCell::new(0),
                };
                i += 7;
                monkeys.insert(monkey.id, monkey);
            }
        }
        monkeys
    }

    fn parse_items(items: &str) -> RefCell<Vec<usize>> {
        let vec = items.split(", ")
            .into_iter()
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        RefCell::new(vec)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &'static str = include_str!("input11_test.txt");

        #[test]
        fn test_solve_part_1() {
            assert_eq!(10605, solve_part_1(TEST_INPUT));
        }
        
        #[test]
        fn do_solve_part_1() {
            assert_eq!(110220, solve_part_1(INPUT));
        }

    }
    
}
