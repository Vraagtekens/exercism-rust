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

    // match n {
    //     0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
    //     1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
    //     2 => format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
    //     _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n - 1)
    // }
}

pub fn sing(start: u32, end: u32) -> String {

    let mut lyrics = String::from(verse(start));
    for i in (end..start).rev() {
        lyrics = format!("{lyrics}\n\n{}", verse(i));
    };

    lyrics

    // if current_verse == end {
    //     verse(end)
    // } else {
    //     verse(current_verse) + "\n" + sing(current_verse - 1, end).as_str()
    // }
}
