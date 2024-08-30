pub fn is_leap_year(year: u64) -> bool {


    // year % 4 == 0 || year % 100 == 0
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)

    // if year % 4 == 0 {
    //     if year % 100 == 0 {
    //         if year % 400 == 0 {
    //             true
    //         } else {
    //             false
    //         }
    //     } 
    //     true
    // } else {
    //     false
    // }
}
