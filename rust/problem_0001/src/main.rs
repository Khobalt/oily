fn main() {
    let mut total = 0;
    for elem in 1..1000 {
        if elem % 3 == 0 || elem % 5 == 0 {
            total += elem;
        }
    }
    println!("{}", total);
}
