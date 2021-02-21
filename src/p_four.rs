use palindrome::is_palindrome;

pub(crate) fn largest_palindrome() -> i32 {
    let mut max = 0;
    for i in 0..=999 {
        for j in 0..=999 {
            let prod = i*j;
            if is_palindrome(prod.to_string()) && prod > max {
                max = prod;
                println!("i = {}, j = {}", i, j);
            }
        }
    }
    return max;
}