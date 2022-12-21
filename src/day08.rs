#[allow(dead_code)]
mod solution {
    const INPUT: &'static str = include_str!("input08.txt");

    fn solve(input: &str) -> usize {
        let (height_map, height, width) = parse(input);
        // println!("{:?}", height_map);
        // println!("height={} width={}", height, width);
        let mut visible_map = vec![vec![false; width]; height];

        // all trees at the edge are visible
        for y in 0..height {
            visible_map[y][0] = true;
            visible_map[y][width - 1] = true;
        }
        for x in 1..width - 1 {
            visible_map[0][x] = true;
            visible_map[height - 1][x] = true;
        }
        // top->down & bottom->up
        for x in 1..(width - 1) {
            let mut max_from_top = height_map[0][x];
            let mut max_from_bottom = height_map[height-1][x];
            
            for i in 1..(height - 1) {
                let y_from_top = i;                
                // println!("y={} x={} height={} max={}", y_from_top, x, height_map[y_from_top][x], max_from_top);
                if height_map[y_from_top][x] > max_from_top {
                    // println!("visible!");
                    visible_map[y_from_top][x] = true;
                }
                if height_map[y_from_top][x] > max_from_top {
                    max_from_top = height_map[y_from_top][x];
                    // println!("new max={}", max_from_top);
                }
                let y_from_bottom = height - i - 1;
                if height_map[y_from_bottom][x] > max_from_bottom {
                    // println!("visible!");
                    visible_map[y_from_bottom][x] = true;
                }
                if height_map[y_from_bottom][x] > max_from_bottom {
                    max_from_bottom = height_map[y_from_bottom][x];
                    // println!("new max={}", max_from_bottom);
                }
            }
        }
        // left->right & right->left
        for y in 1..(height-1) {
            let mut max_from_left = height_map[y][0];
            let mut max_from_right = height_map[y][width-1];
            for i in 1..width - 1 {
                let x_from_left = i;
                if height_map[y][x_from_left] > max_from_left {
                    visible_map[y][x_from_left] = true;
                }
                if height_map[y][x_from_left] > max_from_left {
                    max_from_left = height_map[y][x_from_left];
                }
                let x_from_right = width - i - 1;
                if height_map[y][x_from_right] > max_from_right {
                    visible_map[y][x_from_right] = true;
                }
                if height_map[y][x_from_right] > max_from_right {
                    max_from_right = height_map[y][x_from_right];
                }
            };
        }

        // print_map(&visible_map);
        let mut visible_count = 0;
        for x in 0..width {
            for y in 0..height {
                if visible_map[y][x] {
                    visible_count += 1;
                }
            }
        }
        visible_count
    }

    fn print_map(map: &Vec<Vec<bool>>) {
        for i in 0..map.len() {
            println!("{:?}", map[i])
        };
    }

    fn parse(input: &str) -> (Vec<Vec<u8>>, usize, usize) {
        let mut map: Vec<Vec<u8>> = vec![];
        for (y, line) in input.split("\n").filter(|l| !l.is_empty()).into_iter().enumerate() {
            let row: Vec<u8> = vec![];
            map.push(row);
            for x in 0..line.len() {
                let height_str = &line[x..x + 1];
                let height = height_str.parse::<u8>().expect(height_str);
                map[y].push(height);
            }
        }
        let height = map.len();
        let width = map[0].len();
        (map, height, width)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

        const TEST_INPUT_PARSE: &str = "123
456";

        #[test]
        fn test_parse() {
            let (map, height, width) = parse(TEST_INPUT_PARSE);
            assert_eq!(2, height);
            assert_eq!(3, width);
            assert_eq!(2, map.len());
            assert_eq!(vec![1, 2, 3], map[0]);
            assert_eq!(vec![4, 5, 6], map[1]);
        }

        #[test]
        fn test_solve_part_1() {
            assert_eq!(21, solve(TEST_INPUT))
        }

        #[test]
        fn do_solve_part_1() {
            assert_eq!(1840, solve(INPUT))
        }
    }
}
