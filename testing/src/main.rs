#[cfg(test)]
mod tests{
    #[test]
    fn my_custom_test(){
        assert_eq!(2+2, 4);
        assert_ne!(2 + 2, 3);
    }

    #[test]
    fn my_another_test(){
        assert_eq!(2 + 2, 3);
    }
}
fn main(){
    struct Rectangle{
        a: u64,
        b: u64,
    }

    impl Rectangle {
        fn area(&self) -> u64 {
            self.a * self.b
        }
    }
    
    #[cfg(test)]
    mod tests{
        use super::*;
        #[test]
        fn test_rectangle_area(){
            let rect1 = Rectangle { a: 3, b: 2};
            assert_eq!(rect1.area(), 6);
        }
    }

    pub fn fibonacci(n: i32) -> u64 {
        if n < 0 {
            panic!("{} is negative", n);
        }else if n == 0 {
            panic!("zero is not a right argument to fibonacci()!");
        }else if n == 1 {
            return 1;
        }

        let mut sum = 0;
        let mut last = 0;
        let mut curr = 1;
        for _i in 1..n{
            sum = last + curr;
            last = curr;
            curr = sum;
        }
        sum
    }

    #[cfg(test)]
        mod tests {

            #[test]
            fn test_fibonacci(){
                let number5 = super::fibonacci(6);
                assert_eq!(number5, 8);
            }
        }
}