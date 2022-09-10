fn main() {
    println!("Hello, world!");

    let max: u32 = 1000;

    let mut sum: u32 = 0;

    for num in 1..max {
        if num % 3 == 0 || num % 5 ==0 {
            sum += num;
        }
    }
    println!("Final sum is: {}", sum);
}
