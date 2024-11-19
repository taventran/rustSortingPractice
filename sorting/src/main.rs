use rand::Rng;
mod sorting;
use sorting::{bubble_sort, merge_sort};

fn print_vec(nums: &mut Vec<u8>) {
    for i in 0..nums.len() {
        print!("{} ", nums[i])
    }
    println!();
}

fn generate_vec() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut nums = Vec::new();
    for _ in 0..=10 {
        let num = rng.gen::<u8>();
        nums.push(num);
    }
    nums
}

fn main() {
    let mut nums = generate_vec();
    println!("Printing random numbers for bubble sort:");
    print_vec(&mut nums);
    bubble_sort(&mut nums);
    println!("Printing sorted numbers for bubble sort:");
    print_vec(&mut nums);

    println!();

    nums = generate_vec();
    println!("Printing random numbers for merge sort:");
    print_vec(&mut nums);
    let right = nums.len() - 1;
    merge_sort(&mut nums, 0, right);
    println!("Printing sorted numbers for merge sort:");
    print_vec(&mut nums);
}
