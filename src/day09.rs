mod solution {
    use std::collections::HashSet;

    const INPUT: &'static str = include_str!("input09.txt");

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Position {
        x: i32,
        y: i32,
    }

    fn solve(input: &str) -> usize {
        let moves = parse(input);
        let mut head = Position { x: 0, y: 0 };
        let mut tail = Position { x: 0, y: 0 };
        let mut tail_positions = HashSet::new();
        // println!("head={:?}\n---", head);
        for mv in moves {
            match mv {
                ("U", steps) => move_up(&mut head, &mut tail, steps, &mut tail_positions),
                ("D", steps) => move_down(&mut head, &mut tail, steps, &mut tail_positions),
                ("L", steps) => move_left(&mut head, &mut tail, steps, &mut tail_positions),
                ("R", steps) => move_right(&mut head, &mut tail, steps, &mut tail_positions),
                _ => panic!("Invalid move")
            }
            // println!("head={:?}", head);
        }
        tail_positions.len()
    }

    fn move_up(head: &mut Position, tail: &mut Position, steps: usize, tail_pos: &mut HashSet<Position>) {
        move_horizontal(head, tail, steps, -1, tail_pos)
    }

    fn move_down(head: &mut Position, tail: &mut Position, steps: usize, tail_pos: &mut HashSet<Position>) {
        move_horizontal(head, tail, steps, 1, tail_pos);
    }

    fn move_right(head: &mut Position, tail: &mut Position, steps: usize, tail_pos: &mut HashSet<Position>) {
        move_vertical(head, tail, steps, 1, tail_pos);
    }

    fn move_left(head: &mut Position, tail: &mut Position, steps: usize, tail_pos: &mut HashSet<Position>) {
        move_vertical(head, tail, steps, -1, tail_pos);
    }

    fn move_horizontal(head: &mut Position, tail: &mut Position, steps: usize, dir: i32, tail_pos: &mut HashSet<Position>) {
        for _ in 0..steps {
            head.y = head.y + dir;
            if (head.y - tail.y).abs() > 1 {
                tail.y = head.y - dir;
                tail.x = head.x;
            }
            tail_pos.insert(Position { ..*tail });
        }
    }

    fn move_vertical(head: &mut Position, tail: &mut Position, steps: usize, dir: i32, tail_pos: &mut HashSet<Position>) {
        for _ in 0..steps {
            head.x = head.x + dir;
            if (head.x - tail.x).abs() > 1 {
                tail.x = head.x - dir;
                tail.y = head.y;
            }
            tail_pos.insert(Position { ..*tail });
        }
    }

    fn parse(lines: &str) -> Vec<(&str, usize)> {
        let mut moves = vec![];
        for line in lines.split("\n").filter(|l| !l.is_empty()) {
            let mut split = line.split(" ");
            let dir = split.next().expect("could not extract direction");
            let steps = split.next().expect("could not extract steps").parse::<usize>().expect("could not parse steps");
            moves.push((dir, steps));
        }
        moves
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        #[test]
        fn test_move_up() {
            let mut head = Position { x: 1, y: 1 };
            let mut tail = Position { x: 1, y: 2 };

            move_up(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 1, y: 0 }, head);
            assert_eq!(Position { x: 1, y: 1 }, tail);
        }

        #[test]
        fn test_move_up_with_x_correction() {
            let mut head = Position { x: 1, y: 1 };
            let mut tail = Position { x: 0, y: 2 };

            move_up(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 1, y: 0 }, head);
            assert_eq!(Position { x: 1, y: 1 }, tail);
        }

        #[test]
        fn test_move_up_without_correction() {
            let mut head = Position { x: 2, y: 2 };
            let mut tail = Position { x: 1, y: 2 };

            move_up(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 2, y: 1 }, head);
            assert_eq!(Position { x: 1, y: 2 }, tail);
        }

        #[test]
        fn test_move_up_head_to_tail() {
            let mut head = Position { x: 2, y: 1 };
            let mut tail = Position { x: 2, y: 0 };

            move_up(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 2, y: 0 }, head);
            assert_eq!(Position { x: 2, y: 0 }, tail);
        }

        #[test]
        fn test_move_down() {
            let mut head = Position { x: 1, y: 2 };
            let mut tail = Position { x: 1, y: 1 };

            move_down(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 1, y: 3 }, head);
            assert_eq!(Position { x: 1, y: 2 }, tail);
        }

        #[test]
        fn test_move_down_with_x_correction() {
            let mut head = Position { x: 1, y: 2 };
            let mut tail = Position { x: 0, y: 1 };

            move_down(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 1, y: 3 }, head);
            assert_eq!(Position { x: 1, y: 2 }, tail);
        }

        #[test]
        fn test_move_down_without_correction() {
            let mut head = Position { x: 2, y: 2 };
            let mut tail = Position { x: 1, y: 2 };

            move_down(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 2, y: 3 }, head);
            assert_eq!(Position { x: 1, y: 2 }, tail);
        }

        #[test]
        fn test_move_down_head_to_tail() {
            let mut head = Position { x: 2, y: 0 };
            let mut tail = Position { x: 2, y: 1 };

            move_down(&mut head, &mut tail, 1, &mut HashSet::new());

            assert_eq!(Position { x: 2, y: 1 }, head);
            assert_eq!(Position { x: 2, y: 1 }, tail);
        }


        #[test]
        fn test_solve_part_1() {
            assert_eq!(13, solve(TEST_INPUT));
        }
        
        #[test]
        fn do_solve_part_1() {
            assert_eq!(6175, solve(INPUT));
        }
    }
}
