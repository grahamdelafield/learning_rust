fn get_factors(mut n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();

    let mut i = 2;

    while i < n {
        if n % i == 0 {
            n = n / i;
            res.push(i);
        } else {
            i += 1;
        }
    }
    res.push(n);
    return res;
}

fn main() {
    let ans = get_factors(600851475143);
    println!("{:?}", ans);
    // get_primes(ans);
}
