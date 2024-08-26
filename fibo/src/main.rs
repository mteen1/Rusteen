fn main() {
    println!("Hello, world!");
    let result = fibo(6);
    println!("restult is {result}")
}

fn fibo(n: u32) -> u32{
    if n==0 || n==1{
        1
    }else{
        fibo(n-1)+fibo(n-2)
    }
}