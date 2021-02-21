pub(crate) fn sum_squares(limit: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=limit {
        sum += i*i;
    }
    return sum;
}

pub(crate) fn square_sums(limit: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=limit {
        sum += i;
    }
    let square = sum*sum;
    return square;
}