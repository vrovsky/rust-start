fn main() {
    //Memory managment 
    //Ownership and dropping in memory
    let mut string1 = String::from("academy");
    println!("Print string in main - 1st attempt: {}", string1);
    string1 = print_string(string1);
    println!("Print string in main - 2st attempt: {}", string1);

    fn print_string(s: String) -> String {
        println!("Received string: {}", s);
        s
    }

    //Borrowing - to access data without taking ownership over it
    //Object are being passed by reference using & before type (s: &String)
    let string2 = String::from("Rust lesson");
    println!("Print string2 in main - 1st attemp: {}", string2);
    print_string2(&string2);  //Borrowing here too
    println!("Print string2 in main - 2st attemp: {}", string2);

    fn print_string2(c: &String){
        println!("Recieved string: {}", c);
    }
    
    //if you want to change variable use &mut with type (&mut String)
    let string3 = String::from("Rust lesson");
    println!("Print string3 in main - 1st attemp: {}", string3);
    print_string3(&string3);  //Borrowing here too
    println!("Print string3 in main - 2st attemp: {}", string3);

    fn print_string3(d: &mut String){ //This is not working by memory defence
        println!("Recieved string: {}", d);
    }
}
