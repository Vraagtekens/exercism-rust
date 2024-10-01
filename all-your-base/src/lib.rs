

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    if from_base <= 1 {
        return Err(Error::InvalidInputBase)
    } 
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase)
    }
    if from_base == 2 && number.contains(&2){
        return Err(Error::InvalidDigit(2))
    }

    let numbers: Vec<u32> = remove_leading_zeros(number);

    let sum: u32 = numbers
        .into_iter()
        .rev()
        .enumerate()
        .map(|x| x.1 * from_base.pow(x.0.try_into().unwrap()))
        .sum();

    let mut vec: Vec<u32> = vec![];

    if to_base == 10{
        let digits: Vec<u32> = sum
            .to_string()      
            .chars()          
            .map(|c| c.to_digit(10).unwrap()) 
            .collect(); 

        return Ok(digits)
    }
    

    let mut sum: u32 = sum;
    while sum != 0{
        let remainder = sum % to_base; 
        vec.push(remainder);
        
        sum /= to_base
    }

    if vec.is_empty() {
        return Ok(vec![0]);
    }

    let digits: Vec<u32> = vec.into_iter().rev().collect();
    Ok(digits)
}

fn remove_leading_zeros(vec: &[u32]) -> Vec<u32> {
    let pos = vec.iter().position(|&x| x != 0).unwrap_or(vec.len());
    vec[pos..].to_vec()
}