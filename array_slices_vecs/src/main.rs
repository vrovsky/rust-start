use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    //Array - харнятся в стэке, размер известен во время компиляции, все данные одного типа
    let x: [u8; 3] = [1, 3, 5]; //[1, 3, 5]
    let _y: [u8; 3] = [10; 3];   //[10, 10, 10]
    // x[1] = 100;
    assert_eq!([3, 5], &x[1..]);
    for i in &x {
        println!("{:?}", i);
    }
    for (i, d) in x.iter().enumerate() {
        println!("x[{}] = {}", i, d);
    }
    //Slices - не имеют ownership
    let x1: [u8; 4] = [1, 3, 5, 7];
    println!("{:?}", &x1[0..2]); // Slice - пишется через амперсанд
    println!("{:?}", &x1[..2]); 
    println!("{:?}", &x1[2..4]); 
    println!("{:?}", &x1[2..]); 
    println!("{:?}", &x1[..]); 

    //Collections: Vec- данные храняться в куче. Данные храняться последовательно друг за другом
    let mut some_collection: Vec<u8> = Vec::new();
    some_collection.push(1); //В отличие от массива, можем добавлять неограниченое количество элементов
    some_collection.push(3);
    some_collection.push(7);
    println!("{:?}", some_collection);
    for i in some_collection.iter(){
        println!("{}",i);
    }

    //Hashmap - коллекция из стандартной библиотеки, способ хранения данных: ключ-значение, для защиты от HashDos атак
    let mut hash_start: HashMap<&str, u64> = HashMap::new();
    hash_start.insert("key1", 10);
    hash_start.insert("key2", 20);
    println!("{:?}", x); //{"key1": 10, "key2": 20}
    for (key, val) in hash_start.iter(){
        println!("\"{}\" = {}", key, val);
    }

    //HashSet - данные хранятся в куче, способ хранения данных: набор ключей, HashSet - это HashMap где значение = ()  (tuple)
    // () - пустой tuple, используется в расте в функциях, которые ничего не возвращают, вместо null и подыбным в других яхыках
    let mut hashset_start: HashSet<&str> = HashSet::new();
    hashset_start.insert("key1");
    hashset_start.insert("key2");
    for key in hashset_start.iter(){
        println!("{}", key);
    }
    println!("{:?}", hashset_start);

    //B-TreeMap - данные хранятся в куче, способ хранения B-Tree map
    use std::collections::BTreeMap;
    let mut btree_var: BTreeMap<&str, u64> = BTreeMap::new();
    btree_var.insert("key1", 10);
    btree_var.insert("key2", 20);
    // insert a kay only if it doesn't already exist
    btree_var.entry("key1").or_insert(100);
    for (key, val) in btree_var.iter(){
        println!("{:?} => {:?}", key, val);
    }
    if btree_var.contains_key("key1"){
        //..
    }
    println!("{:?}", btree_var);
}
