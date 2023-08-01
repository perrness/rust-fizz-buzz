fn main() {
    for i in 0..100 {
        if i % 2 == 0 {
            println!("{} Fizz", i);
        } else {
            println!("{} Buzz", i);
        }
    }
}
