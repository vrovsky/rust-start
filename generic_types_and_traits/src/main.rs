use core::ops::Add;
fn main() {
    //Generics - обобщенные типы данных
    //Решает задачу дублирования кода
    //Решает задачу обобщения типов с заданными ограничениями
    //Привносит гибкость и эффективность в написании кода и в дизайн приложения

    pub struct Shape {
        pub x: f32,
        pub y: f32,
    }

    impl Shape {
        pub fn sum(&self) -> f32 { //&self - обязательно, если мы хотим обращаться к структуре Shepe
            self.x + self.y        //& - передача переменных через ссылку
        }
    }
    //В примере выше мы постоянно дублируем f32, ниже используем Generic types
    pub struct Shape1<T>{
        pub x1: T,
        pub y1: T,
    }

    impl<T: Copy> Shape1<T> { //
        pub fn get_x1(&self) -> T {
            self.x1
        }
    }

    //Bigger implementation
    pub struct Shape2<T> {
        pub x2: T,
        pub y2: T,
    }

    impl<T: Copy + Add<Output = T>> Shape2<T> { //Применение нескольких трэйтов идет через +
        pub fn sum(&self) -> T {
            self.x2 + self.y2
        }
    }
    pub struct Shape3<T, U>{
        pub x3: T,
        pub y3: T,
        pub z3: U,
    }

    impl<T: Copy + Add<Output = T>, U: Copy> Shape3<T, U> {
        pub fn sum(&self) -> T {
            self.x3 + self.y3
        }
        pub fn get_z3(&self) -> U {
            self.z3
        }
    }

    //generics с более упрощенным видом
    pub struct Shape4<T, U> {
        pub x4: T,
        pub y4: T,
        pub z4: U,
    }
    impl<T, U> Shape4<T, U>
        where
        T: Copy + Add<Output = T>, //ограничение Trait-ом
        U: Copy,
        {
            pub fn sum(&self) -> T { self.x4 + self.y4 }
            pub fn get_z(&self) -> U { self.z4}
        }

    //Generics в функциях
    pub struct Shape5<T> {
        pub x5: T,
        pub y5: T,
    }

    fn sum<T: Copy + Add<Output = T>>(figure: Shape5<T>) -> T {
        figure.x5 + figure.y5 
    }

    //Пример вызова
    let f: Shape5<i32> = Shape5 {x5: 10, y5: -20};
    let res = sum::<i32>(f);
    // println!("{:?}", f.sum());
    println!("{:?}", res);

    //Traits - абстрактное описание функции, применимым к другим типам(в том числе Generic Types)
    //Аналаги интерфейсов в других языках програмирования
    pub trait Calc{
        fn new() -> Self;
        fn add(&self, x: i32) -> i32;
        fn add2(&self, x: i32, y: i32) -> i32;
        fn change(&mut self, x: i32);
        fn sub_1(&self, x: i32) -> i32 {
            x - 1
        }
    }

    pub trait ShapeCalc<T> {
        fn sum(&self) -> T;
    }

    impl<T: Copy + Add<Output = T>> ShapeCalc<T> for Shape1<T>
    {
        fn sum(&self) -> T{
            self.x1 + self.y1
        }
    }

    impl<T: ShapeCalc<T>> Shape1<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T>,
    {
        pub fn calc(&self, z3: T) -> T{
            z3.sum() - self.x
        }
        pub fn sub(&self, z: &impl ShapeCalc<T>) -> T { //тут отдельно прописываем generic type
            z3.sum() - self.x
        }
    }
}
