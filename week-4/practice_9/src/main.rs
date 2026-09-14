fn main() {
    let mut count = 0;

    for num in 1..21 {
        if num > 10 {
            continue;
        }
        count += 1;
        println!("{}", num);
    }

    println!("Final count: {}", count);
}