const out_of_scope_const: u64 = 10; //can be assigned out of main scope

fn main() {
    let unmutable_variable: u64 = 10;
    let mut mutable_vatible: i64 = 10;
    mutable_vatible = mutable_vatible - 100;

    const in_scope_const: u64 = 20;
    let z = out_of_scope_const * in_scope_const; //const x is assigned out of main scope

    //TUPLES
    fn tuples_function() -> (u8, f32){
        (10, 2.3)
    }
    fn second_tupels() -> (u8, f32){
        let x = (10, 2.3);
        x
    }
    fn third_tuples() -> (u8, f32){
        let (x, y) = (10, 2.3);
        (x, y)
    }
    
    let some_tuple = (10, 2.3);
    println!("u8: {:?}, f32: {:?}", some_tuple.0, some_tuple.1);

    //Перекрытие переменных
    let variable = 10;
    let variable = 30;
    let variable2 = variable;
    println!("{}", variable2);

    //Область видимости констант
    pub const global_const: u64 = 10; // Видимость вообще до другого модуля может дойти

    //ENUM
    pub enum Calc{
        Add,             //атомарный
        Minus(u64, f64), //тюпл
        Mul {            //структура
            x: u64,
            y: f64,
        },
    }

    //ENUM реализует ADT(Abstract data type)
    fn main2() {
        let x: Calc = Calc::Minus(10, 2.3);
        let y = match x {
            Calc::Minus(x, y) => y - x as f64,
            _ => 0.0,
        };
        println!("{:?}", y);
    }
    // with if statment
    fn main3(){
        let x = Some(10);
        if let Some(y) = x {
            println!("{:?}", y);
        } else {
            println!("No result");
        }
    }
    //with match
    fn main3_with_match(){
        let x = Some(10);
        match x {
            Some(y) => println!("{:?}", y),
            _ => println!("No results"), 
        }
    }

    //МНОГОПОТОЧНОСТЬ
    //Rc(Read counter), Arc(Asynchronic read counter), Mutex, Cow

}
