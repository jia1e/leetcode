/*
 * @lc app=leetcode id=150 lang=rust
 *
 * [150] Evaluate Reverse Polish Notation
 */

// @lc code=start

macro_rules! eval_rpn {
    ($op:expr, $stack:expr) => {{
        if let (Some(b), Some(a)) = ($stack.pop(), $stack.pop()) {
            match $op {
                '+' => $stack.push(a + b),
                '-' => $stack.push(a - b),
                '*' => $stack.push(a * b),
                '/' => $stack.push(a / b),
                _ => (),
            }
        }
    }};
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            match token.chars().nth(0).unwrap() {
                op @ ('+' | '-' | '*' | '/') if token.len() == 1 => eval_rpn!(op, stack),
                _ => match token.parse::<i32>() {
                    Ok(n) => stack.push(n),
                    _ => return 0,
                },
            }
        }

        *stack.last().unwrap()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec!["1"], 1),
        (vec!["1", "2", "+", "3", "*"], 9),
        (vec!["4", "13", "5", "/", "+"], 6),
        (
            vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
            ],
            22,
        ),
    ];

    for (tokens, expected) in cases {
        assert_eq!(
            Solution::eval_rpn(tokens.iter().map(|token| token.to_string()).collect()),
            expected
        );
    }
}
