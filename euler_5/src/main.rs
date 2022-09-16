fn main() {

    let start:u32 = 2;
    let end:u32 = 20;

    let mut nums: Vec<u32> = (start..end+1).collect();

    let mut res_values: Vec<u32> = Vec::new();
    
    while nums.len() > 0 {
        let mut current_num = nums[0];
        let mut current_pow = 1;
        while current_num.pow(current_pow+1) < end {
            current_pow += 1
        }
        res_values.push(current_num.pow(current_pow));
        nums.retain(|x| *x % current_num != 0);
        println!("{:?}", nums);
    }
    println!("{:?}", res_values);
    
    let mut cum_prod: u32 = 1;

    for val in res_values {
        cum_prod *= val
    }

    println!("Final value is: {}", cum_prod);
}
