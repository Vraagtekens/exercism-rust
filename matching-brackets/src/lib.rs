pub fn brackets_are_balanced(string: &str) -> bool {

    // let LIST = ['[', ']', '{', '}', '(', ')'];
    // let chars: Vec<char> = string.chars().filter(|x| !LIST.contains(x)).collect();
    let mut stack = Vec::new();

    // println!("{:?}", chars);

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            },
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            },
            _ => continue, // Ignore non-bracket characters
        }
    }

    

    stack.is_empty()
}

