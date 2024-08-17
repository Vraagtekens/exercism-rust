

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }

    let cols = minefield[0].len();

    let mut field: Vec<Vec<char>> = minefield
        .iter()
        .map(|&row| row.chars().collect())
        .collect();

    for i in 0..rows {
        for j in 0..cols {
            if field[i][j] == '*' {
                continue;
            }

            let mut count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        if field[ni as usize][nj as usize] == '*' {
                            count += 1;
                        }
                    }
                }
            }

            if count > 0 {
                field[i][j] = char::from_digit(count, 10).unwrap();
            }
        }
    }

    field.into_iter()
    .map(|row| row.into_iter().collect::<String>())
    .collect()
}
