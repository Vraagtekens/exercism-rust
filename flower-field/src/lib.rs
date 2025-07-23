pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for (index, row) in garden.iter().enumerate() {
        let row = row
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let c = *c as char;

                if c == '*' {
                    return c;
                }

                // get sub slice and count every "flower"
                let min = index.saturating_sub(1);
                let max = (index + 1).min(garden.len() - 1);
                let result = garden.get(min..=max);
                match result {
                    Some(result) => {
                        let mut sum = 0;

                        for r in result {
                            let min_x = i.saturating_sub(1);
                            let max_x = (i + 1).min(r.len() - 1);
                            let r = r.get(min_x..=max_x);

                            let num = match r {
                                Some(r) => {
                                    r.as_bytes().iter().filter(|c| **c as char == '*').count()
                                }
                                None => 0,
                            };

                            sum += num;
                        }

                        if sum == 0 {
                            ' '
                        } else {
                            *sum.to_string().as_bytes().iter().next().unwrap() as char
                        }
                    }
                    None => '·',
                }
            })
            .collect::<String>();

        list.push(row);
    }

    list
}
