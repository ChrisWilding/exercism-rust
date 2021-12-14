pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let opening = stack.pop();
                let matching = opening
                    .map(|o| match c {
                        ')' => o == '(',
                        ']' => o == '[',
                        '}' => o == '{',
                        _ => false,
                    })
                    .unwrap_or(false);
                if !matching {
                    return false;
                }
            }
            _ => continue,
        };
    }

    stack.len() == 0
}
