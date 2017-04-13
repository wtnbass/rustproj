pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        struct T {
            input: (i32, i32),
            output: i32,
        }
        let test = vec![
            T{input: (1, 2), output: 3},
            T{input: (-1, 2), output: 1},
            T{input: (1, -2), output: -1},
            T{input: (-1, -2), output: -3},
        ];
        for t in &test {
            let (a, b) = t.input;
            assert_eq!(t.output, add(a, b));
        }
    }
}
