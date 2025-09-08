use std::{time::Instant, vec};

fn main() {
    const N: usize = 1e8 as usize;
    const NSQ: usize = N.isqrt();
    
    let mut arr = vec![true; N + 1];
    
    let start = Instant::now();
    for i in 2..=NSQ {
        if arr[i] {
            let mut j = i.pow(2);
            while j <= N {
                arr[j] = false;
                j = j + i;
            }
        }
    }

    let mut count = 0;
    let mut sum = 0;
    let mut primes = Vec::new();
    for i in 2..=N {
        if arr[i] {
            count = count + 1;
            sum = sum + i;
            primes.push(i);
        }
    }

    let end = start.elapsed();
    print!("Execution Time: {} seconds\t", end.as_secs());
    print!("{} Primes Found\t", count);
    println!("Sum of Primes: {}", sum);

    println!("Top 10 Largest Primes:");
    for i in (1..=10).rev() {
        println!("{}.\t {}", i, primes[primes.len() - i]);
    }
}
