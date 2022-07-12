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

    //loop
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

    //while
    fn ping_n_times2(n: usize){
        let mut counter = 0;

        while counter < n {
            println!("Ping attempt with while: {}", counter);

            counter += 1;
        }
    }
    ping_n_times2(4);

    //for
    fn ping_n_times3(n: usize) {
        for i in 0..n {
            println!("Ping attempt: {}", i);
        }
    }
    ping_n_times3(4);

    //scruct
    struct User {
        username: String,
        email: String,
        age: u64,
        active: bool,
    }

    let mut user1 = User{
        email: String::from("user1@mail.com"),
        username: String::from("ping_machine"),
        active: true,
        age: 42,
    };
    user1.age = 20;

    let user2 = User{
        email: String::from("another@mail.com"),
        username: String::from("another_user"),
        ..user1
    };

    //Tuple structs
    struct Color(i32, i32, i32);

    let color: Color = Color(255, 128, 128);
    let r = color.0;
    let g = color.1;
    let b = color.2;

    //implemetatio0n of ctructure
    struct PingMachine{
        times_to_ping: usize,
    }

    impl PingMachine {
        fn get_num_times_to_ping(&self) -> usize {
            self.times_to_ping
        }

        fn ping(&mut self) {
            if self.times_to_ping > 0 {
                println!("ping");
                self.times_to_ping -= 1;
            } else {
                println!("No more pings!");
            }
        }
    }

    let mut ping_machine = PingMachine {
        times_to_ping: 3,
    };
    
    ping_machine.ping();
    ping_machine.ping();
    ping_machine.ping();
    ping_machine.ping();

}
