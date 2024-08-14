pub fn verse(n: u32) -> String {

    let mut sentence: String = String::from("");

    if n > 2 {
        sentence = format!(
            "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.", n, n, n - 1
        );
    } else if n == 2{
        sentence = format!(
            "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.", n, n, n - 1
        );
    } else if n == 1 {
        sentence = format!(
            "{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.", n, n
        );
    } else if n == 0 {
        sentence = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.");
    }

    sentence
}

pub fn sing(start: u32, end: u32) -> String {

    let mut lyrics = String::from(verse(start));
    for i in (end..start).rev() {
        lyrics = format!("{lyrics}\n\n{}", verse(i));
    };

    lyrics
}
