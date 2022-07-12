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
    println!("Result of sum_values2 function: {}", sum_values2(15, 23));
}
