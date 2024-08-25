pub fn find(array: &[i32], key: i32) -> Option<usize> {
    
    // array.iter().map(|x| {
    //     if x > &key {

    //     }
    // });

    let mut length = array.len() / 2;
    let mut count = 0;
    // println!("{:?}", length);

    loop {
        if array[length] == key{
            break Some(length);
        }
        if length == 1{
            break Some(0);
        }

        // if len % 2 == 0 {
        //     // Even length: average of two middle elements
        //     (numbers[mid - 1] as f32 + numbers[mid] as f32) / 2.0
        // }

        // let (left, right) = array.split_at(length);
        // println!("{:?}", length);
        // println!("{:?}", right);
        // println!("{:?}", left);
        if array[length] > key {
            println!("{:?}", length);
            length = length - length / 2;
        } else if array[length] < key{
            length = length + length / 2;
        } else {
            break None;
        }

        count+= 1;
        if count > array.len(){
            break None;
        }
    }
}
