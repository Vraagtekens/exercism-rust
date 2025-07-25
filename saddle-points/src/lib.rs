pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, num) in row.iter().enumerate() {
            let row_maxes: Vec<u64> = input.iter().map(|row| *row.iter().max().unwrap()).collect();

            let mut col_mins = vec![u64::MAX; input[0].len()];
            for row in input {
                for (col, &val) in row.iter().enumerate() {
                    col_mins[col] = col_mins[col].min(val);
                }
            }

            if *num == row_maxes[row_index] && *num == col_mins[column_index] {
                result.push((row_index, column_index));
            }
        }
    }

    result
}
