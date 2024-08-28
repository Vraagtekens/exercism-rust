const NAMES: [&str; 12] = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", 
"Joseph", "Kincaid", "Larry"];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {

    let x: Vec<&str> = diagram.split("\n").collect();
    let mut list = Vec::new();
    let index = NAMES.into_iter().position(|x| x == student).unwrap();
    let i = match student {
        "Alice" => 0,
        "Bob" => 2, 
        "Charlie" => 4, 
        "David" => 6, 
        "Eve" => 8, 
        "Fred" => 10, 
        "Ginny" => 12, 
        "Harriet" => 14, 
        "Ileana" => 16, 
        "Joseph" => 18, 
        "Kincaid" => 20, 
        "Larry" => 22,
        _ => 0
    };

    println!("{:?}", x);

    for sentence in x {
        let z: Vec<_> = sentence.trim().chars().map(|x| {
            match x {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => ""
            }
        }).collect();

        list.push(z[i]);
        list.push(z[i + 1]);
    }

    list
}
