pub fn get_diamond(c: char) -> Vec<String> {
    let mut arr: Vec<String> = vec![];
    let position = c as u32 - 'A' as u32;

    for (i, c) in ('A'..=c).enumerate() {
        if i != 0 {
            let mut x = (0..position - 1).map(|_| " ").collect::<String>();
            x.insert(i - 1, c);
            let first = x.chars().rev().collect::<String>();

            arr.push(format!("{first} {x}"));
        } else {
            let x = (0..position).map(|_| " ").collect::<String>();

            arr.push(format!("{x}{c}{x}"));
        }
    }

    let mut right = arr.clone();
    for i in (0..arr.len() - 1).rev() {
        right.push(arr[i].to_string());
    }

    right
}
