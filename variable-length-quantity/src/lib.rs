#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let list: Vec<Vec<u8>> = values
        .iter()
        .map(|num| {
            let bin_str = format!("{num:b}");
            let bin_str = {
                let mut x: String = bin_str.chars().rev().collect();

                while x.len() % 7 != 0 {
                    x.push('0');
                }

                x.chars().rev().collect::<String>()
            };

            let chunks: Vec<&[u8]> = bin_str.as_bytes().chunks(7).collect();

            let mut lol: Vec<u8> = vec![];
            for (i, chunk) in chunks.iter().enumerate() {
                let is_last = i == chunks.len() - 1;

                let xd = str::from_utf8(chunk).unwrap();
                let mut xd: String = xd.chars().rev().collect();
                if is_last {
                    xd.push('0');
                } else {
                    xd.push('1');
                }

                let xd: String = xd.chars().rev().collect();
                let bin = u8::from_str_radix(&xd, 2).unwrap();
                lol.push(bin);
            }

            lol
        })
        .collect();

    list.into_iter().flatten().collect()

    // vec![]
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    // todo!("Convert the list of bytes {bytes:?} to a list of numbers")

    let mut list: Vec<u32> = vec![];
    let mut value = 0;
    for byte in bytes {
        // println!("{:b}", byte);
        let bin_str = format!("{byte:b}");

        println!("{bin_str}");

        let bin_str = {
            let mut x: String = bin_str.chars().rev().collect();

            while x.len() % 7 != 0 {
                x.push('0');
            }

            let huh = x.chars().rev().collect::<String>();
        };
    }

    Ok(list)
}
