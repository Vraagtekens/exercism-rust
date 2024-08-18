
pub fn build_proverb(list: &[&str]) -> String {
    let length = list.len();
    if length == 0 {
        return String::new();
    }

    let mut story = String::new();
    for i in 1..length {
        story.push_str(&format!("For want of a {} the {} was lost.\n", 
            list[i-1], list[i]));
    }
    story.push_str(&format!("And all for the want of a {}.", list[0]));

    story
}
