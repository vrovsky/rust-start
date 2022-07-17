fn main() {
    //Error handling
    //Обработка ошибок требуется во время компиляции(нельзя скомпилировать)
    //Типы ошибок: 
    //Recoverable errors(file not found, network connections is closed, container is empty)

    //Unrecoverable errors(accessing the location out of array bounds and --->
    //---->Errors that can't be handled by the proggrammer:
    //        panic! macro(prints failure message, unwinds and cleans up the stack and quit)
    //Abort on panic! in Cargo.toml: [profile.releas] panic = 'abort'
    
    panic!("Opps, the code is on fire");

    //Recoverable errors - most errors are easy to recover
    pub enum Option<T>{
        None,
        Some(T),
    }
    //Usage: initialization of values when you don't want for vatiable to have definition on initilization
    //Usage: partial functions, whe in 1 function there can be 3, 4 or 5 parameters and can be called with 3 parameters
    //Usage: simple errors(file not found, file can't be open)

    let x: Option<u64> = Some(123_456_789);
    assert_eq!(x.is_some(), true);

    let x: Option<u64> = None;
    assert_eq!(x.is_some(), false);

    Result<T, E>
    enum Result<T, E>{
        Ok(T),
        Err(E),
    }

    //T and E are generic parameters
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hey_rustaceans.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Can't open the file: {:?}", error),
    };

    //Or more error information
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hey_rustaceans.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    //Unwrap
    let f = File::open("hey_rustaceans.txt").unwrap();
    let some_empty_data = None;
    some_empty_data.unwrap();
    some_empty_data.expect("The data is empty!");

}
