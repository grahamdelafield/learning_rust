fn main() {
    let start: u32 = 2;
    let end: u32 = 10;

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }


    let mut nums: Vec<u32> = (start..end+1).collect();
    println!("{:?}", nums);
    let mut found_vals: Vec<u32> = Vec::new();
    
    for num in &nums {
        if !found_vals.contains(num) {
            let mut current_power: u32 = 1;
            while num.pow(current_power+1) < end {
                current_power += 1
            }
            found_vals.push(num.pow(current_power));
            let index = nums.iter().position(|&x| x%num == 0).unwrap();
            println!("{:?}", index);
            // println!("{} has maximum power of {} -> {:?}", num, current_power, found_vals);
        
        }
    }

    // println!("{:?}", nums);
}
