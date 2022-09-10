fn main() {
    println!("Hello, world!");

    fn fib(n: u32) -> u32 {
        if n == 1 || n == 2 {
            return n
        }
        else {
            let p1 = fib(n-1);
            let p2 = fib(n-2);

            return p1 + p2
        }
    }

    
    let mut res: Vec<u32> = Vec::new();
    
    let mut digit: u32 = 1;
    let mut current_ans = fib(1);
    
    while current_ans < 4000000 {
        res.push(current_ans);
        digit += 1;
        current_ans = fib(digit);
    }
    println!("{:?}", res);

    let mut sum = 0;

    for num in res {
        if num % 2 == 0 {
            sum += num
        }
    }

    println!("Final sum is {}", sum);
}
