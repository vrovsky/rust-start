fn main() {
    //Closures(Lambdas)Лямбда - анонимная функция
    let closure1 = || 10;
    // let closure2 = |x| 10 * x;
    let closure3 = |x: i32| -> f32 { (10* x) as f32 / 2.0}; 

    //Функции как переменные
    fn my_func() -> i32 {
        10
    }
    let f1 = my_func;
    println!("Result: {:?}", f1());
}
