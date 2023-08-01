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
        if i % 3 == 0 && i % 5 == 0 {
            fizzbuzz.push(format!("{}{} ", fizz, buzz));
        } else if i % 3 == 0 {
            fizzbuzz.push(format!("{} ", fizz));
        } else if i % 5 == 0 {
            fizzbuzz.push(format!("{} ", buzz));
        } else {
            fizzbuzz.push(format!("{} ", i));
        }
    }

    fizzbuzz
}
