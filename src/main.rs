mod math;

fn euler_one(num: u32) -> u32 {
    // Solves Project Euler 1
    let mut total = 0;
    for n in 1..num {
        if n % 3 == 0 {
            total += n;
        } else if n % 5 == 0 {
            total += n
        }
    }
    return total;
}

fn euler_two(num: i128) -> i128 {
    let mut count = 1;
    let mut fib = 0;
    let mut total = 0;
    while fib < num {
        fib = math::fibonacci(count);

        if fib % 2 == 0 {
            total += fib;
        }
        count += 1;
    }
    return total;
}

fn euler_six(num: i64) -> i64 {
    // Solver Project Euler 6
    let mut sum_of_squares = 0;
    let mut sum = 0;
    for n in 1..num {
        let square = n * n;
        sum += n;
        sum_of_squares += square;
    }
    let square_of_sum = sum * sum;
    let result = square_of_sum - sum_of_squares;
    return result;
}

fn main() {
    println!("The solution to Euler 1 is {}", euler_one(1000));
    println!("The solution to Euler 2 is {}", euler_two(4000000));
    println!("The solution to Euler 6 is {}", euler_six(101));
}
