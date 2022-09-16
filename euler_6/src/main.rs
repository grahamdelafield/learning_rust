fn main() {

    let mut sum_squares: u64 = 0;
    let mut square_sum: u64 = 0;

    for num in 1..101 {
        sum_squares += num*num;
        square_sum += num;
    }

    square_sum = square_sum*square_sum;

    let res = square_sum - sum_squares;

    println!("Difference between square sum and summed squares is {}", res);
}
