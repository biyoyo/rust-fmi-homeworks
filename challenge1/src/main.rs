//first task for course
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1
    {
        1
    }
    else
    {
       let mut prev: u32 = 1;
       let mut result: u32 = 1;
       let mut n = n;

       while n != 1
       {
           result += prev;
           prev = result - prev;
           n -= 1;
       }
       result
    }
}

fn main() {
    for i in 0..21
    {
        println!("{}", fib(i));
    }
}
