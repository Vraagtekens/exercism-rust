pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let fallback = vec![];
    let mut result = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, num) in row.iter().enumerate() {
            //

            let east = if column_index == 0 {
                &0
            } else {
                row.get(column_index.saturating_sub(1)).unwrap_or(&0)
            };

            let west = row.get(column_index + 1).unwrap_or(&0);

            let north = if row_index == 0 {
                &999
            } else {
                let row = input.get(row_index.saturating_sub(1)).unwrap_or(&fallback);

                row.get(column_index).unwrap_or(&999)
            };

            let south = {
                let row = input.get(row_index + 1).unwrap_or(&fallback);

                row.get(column_index).unwrap_or(&999)
            };

            println!("({row_index},{column_index})");
            println!("{num} > {east} = {}", num > east);
            println!("{num} > {west} = {}", num > west);
            println!("{num} < {north} = {}", num < north);
            println!("{num} < {south} = {}", num < south);

            if num >= east && num >= west && num <= north && num <= south {
                //
                result.push((row_index, column_index));
            }
        }
    }

    result
}
