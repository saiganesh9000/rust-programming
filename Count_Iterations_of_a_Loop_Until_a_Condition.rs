fn main() {
    let mut count = 0;
    let mut num = 1;

    while num <= 100 {
        if num % 7 == 0 {
            break;
        }
        count += 1;
        num += 1;
    }

    println!("Loop ran {} times before hitting a number divisible by 7.", count);
}
