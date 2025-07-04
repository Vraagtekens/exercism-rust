use core::fmt;
use std::fmt::Display;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = vec![];

        for index in 0..row_count as usize {
            let row: Vec<u32> = (0..=index)
                .map(|i| {
                    if i == 0 {
                        return 1;
                    }

                    let result = rows.get(index - 1);
                    match result {
                        Some(previous_row) => {
                            let res = previous_row.get((i - 1)..=i);
                            match res {
                                Some(x) => x.iter().sum(),
                                None => 1,
                            }
                        }
                        None => 1,
                    }
                })
                .collect();

            rows.push(row);
        }

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

impl Display for PascalsTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rows.iter().for_each(|row| {});

        
        write!(f, "({:?})", self.rows)
    }
}
