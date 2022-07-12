fn main() {
    fn sum_and_print_result(a: u32, b: u32) {
        println!("The value of a is: {}", a);
        println!("The value of b is: {}", b);
        println!("The sum of a and b is: {}", a + b);
    }  
    sum_and_print_result(25, 29);

    fn sum_values(a: u32, b: u32) -> u32 {
        let sum = a + b;
        return sum;
    }
    println!("Result of sum_values function: {}", sum_values(12,14));   
    
    fn sum_values2(a: u32, b: u32) -> u32 {
        a + b
    }
    //function without return keyword
    println!("Result of sum_values2 function: {}", sum_values2(15, 23));

    fn number_checker(number: u32){
        if number > 0 {
            println!("The number {} is greater than zero", number);
        } else if number < 0 {
            println!("The number {} is less than zero", number);
        } else {
            println!("The number {} is equal to zero", number);
        }            
    }
    number_checker(55);

    //while
    fn ping_n_times(n: usize) {
        let mut counter = 0;

        let result = loop {
            counter += 1;
            println!("ping");
    
            if counter == n {
                break counter;
            }
        };
    println!("We pinged {} times", result);
    }  

    ping_n_times(10);

    fn ping_n_times2(n: usize){
        let mut counter = 0;

        while counter < n {
            println!("Ping attempt: {}", counter);

            counter += 1;
        }
    }
    ping_n_times2(4);
}