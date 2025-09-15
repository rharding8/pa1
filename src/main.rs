use std::time::Instant;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;

fn main() {
    let start = Instant::now();

    let par_primes = par_prime_finder(1e8 as usize, 8);

    let end = start.elapsed();

    print!("Execution Time: {} seconds\t", end.as_secs());
    print!("{} Primes Found\t", par_primes.len());
    println!("Sum of Primes: {}", par_primes.iter().sum::<usize>());

    println!("Top 10 Largest Primes:");
    for i in (1..=10).rev() {
        println!("{}.\t {}", i, par_primes[par_primes.len() - i]);
    }
}

pub fn seq_prime_finder(n: usize) -> Vec<usize> {
    let nsq: usize = n.isqrt();
    
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    
    for i in 2..=nsq {
        if sieve[i] {
            let mut j = i.pow(2);
            while j <= n {
                sieve[j] = false;
                j = j + i;
            }
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

pub fn par_prime_finder(n: usize, threads: usize) -> Vec<usize> {
    ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();
    let nsq: usize = n.isqrt();
    
    let mut base_sieve = vec![true; nsq + 1];
    
    for i in 2..=nsq {
        if base_sieve[i] {
            let mut j = i.pow(2);
            while j <= nsq {
                base_sieve[j] = false;
                j = j + i;
            }
        }
    }

    let base_primes: Vec<usize> = (2..=nsq).filter(|&i| base_sieve[i]).collect();

    let mut par_sieve = vec![true; n + 1];
    par_sieve[0] = false;
    par_sieve[1] = false;

    let chunk_size = (n + 1) / threads + 1;
    par_sieve
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(i, chunk)| {
            let start = i * chunk_size;

            for &p in &base_primes {
                let mut multiple = if start <= p {
                    p * p
                } else {
                    let r = (start + p - 1) / p;
                    r * p
                };

                while multiple < start + chunk.len() && multiple <= n {
                    if multiple >= start {
                        chunk[multiple - start] = false;
                    }
                    multiple += p;
                }
            }
        });

    par_sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}