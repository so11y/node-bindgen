use std::time::SystemTime;

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let start = SystemTime::now();
    println!("{}", fibonacci(40) + fibonacci(10),);
    println!("end {:?}", start.elapsed().unwrap());
}
