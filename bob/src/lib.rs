pub fn reply(message: &str) -> &str {

    if message.trim().is_empty() {
        "Fine. Be that way!"
    } else if 
        message == message.to_uppercase() && 
        message.trim_end().ends_with('?') && 
        message.chars().any(|c| c.is_alphabetic()){
        "Calm down, I know what I'm doing!"
    } else if message == message.to_uppercase() && message.chars().any(|c| c.is_alphabetic()){
        "Whoa, chill out!"
    } else if message.trim_end().ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
