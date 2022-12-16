mod solution {

    fn solve() -> &'static str {
        include_str!("input00.txt").trim()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_input() {
            let result = solve();
            assert_eq!(result, "Hello AoC in Rust!");
        }
    }
}
