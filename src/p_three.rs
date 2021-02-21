use std::collections::HashSet;

pub(crate) fn prime_facts() {
    let mut number: i64 = 600851475143;
    let mut set = HashSet::new();

    for i in 2..number {
        while number % i == 0 {
            set.insert(i);
            number = number / i;
        }
    }
    if number > 2 {
        set.insert(number);
    }

    for x in &set {
        println!("{}", x);
    }
}