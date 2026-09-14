fn main() {
    let mut x = 0;

    loop {
        println!("{}", x);
        if x == 15 {
            break;
        }
        x += 1;
    }

    println!("Broke out of the loop at x = {}", x);
}