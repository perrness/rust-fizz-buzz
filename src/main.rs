fn main() {
    let fizzbuzz = fizz_buzz(100);
    for val in fizzbuzz {
        print!("{}", val);
    }
}

fn fizz_buzz(i: u32) -> Vec<String> {
    let mut fizzbuzz = Vec::new();

    let fizz = "Fizz";
    let buzz = "Buzz";

    for i in 1..i {
        match (i % 3, i % 5) {
            (0, 0) => fizzbuzz.push(format!("{}{} ", fizz, buzz)),
            (0, _) => fizzbuzz.push(format!("{} ", fizz)),
            (_, 0) => fizzbuzz.push(format!("{} ", buzz)),
            _ => fizzbuzz.push(format!("{} ", i)),
        }
    }

    fizzbuzz
}
