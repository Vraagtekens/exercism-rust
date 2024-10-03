
#[macro_export]
macro_rules! hashmap {
    () => {{
        ::std::collections::HashMap::new()
    }};

    (,) => {
        compile_error!("Trailing comma is not allowed.");
    };
  
    ($( $key:expr => $value:expr ),*  $(,)?) => {
        {
            use ::std::collections::HashMap; 
            let mut temp_hash = HashMap::new(); 
            $(
                temp_hash.insert($key, $value);
            )*

            temp_hash
        }
    };
}