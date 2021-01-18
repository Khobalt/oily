/*  The prime factors of 13195 are 5, 7, 13 and 29.
 *
 *  What is the largest prime factor of the number 600851475143 ?
 *
 */
fn main() {
    //find a big factor, test it for primality, repeat until half-way down
    let num: i64 = 600851475143;
    for i in (0..((num as f64).sqrt() as i64)).rev() {
        //If we have a factor
        if num % i == 0 {
            //Check if it's prime
            let mut its_prime = true;
            for j in 2..i / 2 {
                if i % j == 0 {
                    its_prime = false;
                    break;
                }
            }
            if its_prime {
                println!("{}", i);
                return;
            }
        }
    }
    println!("Couldn't find a prime factor.")
}
