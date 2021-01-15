fn main() {
    let mut sum_of_evens = 0;
    let mut first_term = 1;
    let mut second_term = 2;

    let mut sum;
    while second_term < 4000001 {
        sum = first_term + second_term;
        first_term = second_term;
        second_term = sum;
        if second_term % 2 == 0 {
            sum_of_evens += second_term;
        }
    }


    println!("{}", sum_of_evens);
}
