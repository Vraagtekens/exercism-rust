pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }

    let mut list: Vec<Vec<u32>> = (0..size)
        .map(|_| vec![0; size.try_into().unwrap()])
        .collect();

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut x_min: usize = 0;
    let mut y_min: usize = 1;

    let mut index: usize = (size - 1) as usize;
    let mut plus: bool = true;

    for num in 1..=(size.pow(2)) {
        if let Some(row) = list.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                *cell = num;
            }
        }

        if y == index && x == index {
            plus = false;
        } else if y == y_min && !plus {
            plus = true;
            index -= 1;
            x_min += 1;
            y_min += 1;
        }

        match plus {
            true => {
                if x == index {
                    y += 1
                } else {
                    x += 1
                }
            }
            false => {
                if x == x_min {
                    y = y.saturating_sub(1)
                } else {
                    x = x.saturating_sub(1)
                };
            }
        }
    }

    list
}
