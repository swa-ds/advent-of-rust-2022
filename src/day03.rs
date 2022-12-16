#[allow(dead_code,unused_variables)]
mod solution {
    use std::collections::hash_map::RandomState;
    use std::collections::hash_set::Intersection;
    use std::collections::HashSet;

    const INPUT: &'static str = include_str!("input03.txt");

    fn solve_part_1(input: &str) -> usize {
        let it = input.split("\n").into_iter().filter(|i| !i.is_empty());
        let mut sum = 0;
        for items in it {
            let mut left = HashSet::new();
            let mut right = HashSet::new();

            let middle = items.len() / 2;
            for (i, c) in items.chars().enumerate() {
                if i < middle {
                    left.insert(c);
                } else {
                    right.insert(c);
                }
            }
            let intersection = left.intersection(&right);
            sum += get_prio(intersection);
        }
        sum as usize
    }

    fn solve_part_2(input: &str) -> usize {
        let mut iter = input.split("\n").into_iter().filter(|i| !i.is_empty());
        let mut result = 0;
        let mut it = iter.next();

        while !it.is_none() {
            let r1 = to_set(it);
            let r2 = to_set(iter.next());
            let r3 = to_set(iter.next());

            let inters = r1.intersection(&r2);
            let mut hs: HashSet<char> = HashSet::new();
            inters.into_iter().for_each(|c| { hs.insert(*c); });

            let inters = hs.intersection(&r3);
            result += get_prio(inters);

            it = iter.next();
        }

        result as usize
    }

    fn get_prio(mut intersection: Intersection<char, RandomState>) -> u32 {
        let common_item = intersection.next();
        if intersection.next().is_some() { panic!("Only one common item expected, but found more!") };
        match *common_item.unwrap() as u32 {
            n if n > 96 => n - 96,
            n => n - 38,
        }
    }

    fn to_set(str: Option<&str>) -> HashSet<char> {
        let mut set = HashSet::new();
        str.expect("Should be present!").chars().for_each(|c| { set.insert(c); });
        set
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT : &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        #[test]
        fn test_solve_part_1() {
            assert_eq!(157, solve_part_1(TEST_INPUT))
        }

        #[test]
        fn test_solve_part_2() {
            assert_eq!(70, solve_part_2(TEST_INPUT))
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(7980, solve_part_1(INPUT));
        }

        #[test]
        fn do_solve_part_2() {
            assert_eq!(2881, solve_part_2(INPUT));
        }
    }
}
