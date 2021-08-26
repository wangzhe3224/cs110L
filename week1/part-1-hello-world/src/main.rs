fn main() {
    println!("Hello, world!");
    let n: i32 = 1;
    let mut m = 0;
    m = m + 1;
    let s = "Hello, ";
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(2);
    let mut arr: [i32; 4] = [1,2,3,4];
    println!("Array: {}", arr[0]+arr[1]);

    for i in v.iter() {
        println!("{}", i)
    }

    println!("Fib(10) = {}", fib(10))
}

fn sum(a: i32, b: i32) -> i32 {
    a+b
}

fn fib(n: i32) -> i32 {
    if n <= 1 {n} 
    else {fib(n-1)+fib(n-2)}
}
