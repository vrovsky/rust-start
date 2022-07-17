fn main() {
    //Error handling
    //Обработка ошибок требуется во время компиляции(нельзя скомпилировать)
    //Типы ошибок: 
    //Recoverable errors(file not found, network connections is closed, container is empty)

    //Unrecoverable errors(accessing the location out of array bounds and --->
    //---->Errors that can't be handled by the proggrammer:
    //        panic! macro(prints failure message, unwinds and cleans up the stack and quit)
    //Abort on panic! in Cargo.toml: [profile.releas] panic = 'abort'
    
    // panic!("Opps, the code is on fire");

    //Recoverable errors - most errors are easy to recover
    pub enum Option<T>{
        None,
        Some(T),
    }
    //Usage: initialization of values when you don't want for vatiable to have definition on initilization
    //Usage: partial functions, whe in 1 function there can be 3, 4 or 5 parameters and can be called with 3 parameters
    
}
