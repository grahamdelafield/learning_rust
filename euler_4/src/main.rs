fn main() {

    let mut palindromes: Vec<u64> = Vec::new();

    for val1 in (100..999).rev() {
        for val2 in (100..999).rev() {
            let product: u64 = val1 * val2;
            let forward: String = product.to_string();
            let reverse: String = forward.chars().rev().collect();

            if forward == reverse {
                palindromes.push(product);
                }
        }
    }

    println!("Maximum prodcut is: {:?}", palindromes.iter().max().unwrap());
}
