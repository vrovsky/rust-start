#[cfg(test)]
mod tests{
    #[test]
    fn my_custom_test(){
        assert_eq!(2+2, 4);
        assert_ne!(2 + 2, 3);
    }

    // #[test]
    // fn my_another_test(){
    //     assert_eq!(2 + 2, 3);
    // }
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
}