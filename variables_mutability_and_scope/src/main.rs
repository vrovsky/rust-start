const x: u64 = 10; //can be assigned out of main scope

fn main() {
    let unmutable_variable: u64 = 10;
    let mut mutable_vatible: i64 = 10;
    mutable_vatible = mutable_vatible - 100;

    const y: u64 = 20;
    let z = x * y; //const x is assigned out of main scope
}
