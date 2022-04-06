// function that returns factorial of a number
fn factorial(n: u64) -> u64 {
    // (1..=n).product()

    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    for _num in 1..=1000000 {
        factorial(20);
    }
}
