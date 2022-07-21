fn main() {
    //Closures(Lambdas)Лямбда - анонимная функция
    let _closure1 = || 10;
    // let closure2 = |x| 10 * x;
    let _closure3 = |x: i32| -> f32 { (10* x) as f32 / 2.0}; 

    //Функции как переменные
    fn my_func() -> i32 {
        10
    }
    let f1 = my_func;
    println!("Result: {:?}", f1());
    
    let f2 = || 11;
    println!("Result: {:?}", f2());

    //Closures with args
    let f3 = |i: i32| -> i32 {10 + i};
    println!("Result: {:?}", f3(21));

    //Function Traits
    //Функция как параметр
    fn my_func2<F>(f: F) -> i32
    where
        F: FnOnce(i32) -> i32,
    {
        5 * f(30)
    }
    println!("Result: {:?}", my_func2(f3));

    println!(
        "Result: {:?}",
        my_func2(|x| {
            let y = 10;
            x * y + x
        })
    );
    
    let x = vec![10, 20, 30];
    let z = |y| x.iter().fold(y, |x1, x2| x1 * x2);
    println!("Result: {:?} {:?}", x, z(3));

    //Функции как тип
    struct MyType<F>
    where   F: FnOnce(i32) -> i32,
    {
        x: F,
        y: i32,
    }
    let t = MyType { x: f3, y: 32};//f3 - функция
    println!("result: {:?}", (t.x)(t.y));

    //Итераторы
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let y = x.iter();
    println!("{:?}", y);

    let y: i32 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    .iter()
    .filter(|x| *x % 2 != 0)
    .map(|i| i * i)
    .filter(|x| *x < 30)
    .sum();
    println!("Result: {:?}", y);

    //Итераторы:
    //map - генератор
    //fold - агрегатор
    //collect - агрегатор
    //filter - потребитель
}
