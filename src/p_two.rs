pub(crate) fn even_fibs() -> i32 {
    let limit = 4000000;
    let mut index = 0;
    let mut curr = fibonacci(index);
    let mut sum = 0;

    while curr < limit {
        if curr % 2 == 0 {
            println!("Curr: {}", curr);
            sum += curr;
            println!("Sum: {}", sum);
        }
        index += 1;
        curr = fibonacci(index);
    }
    return sum;
}

fn fibonacci(n: i32) -> i32 {
    return if n < 2 {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    };
}