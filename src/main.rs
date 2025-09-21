use std::time::Instant;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::io::Error;

fn main() -> Result<()> {
    let mut file = File::create("output.txt")?;
    let start = Instant::now();

    let par_primes = par_prime_finder(1e8 as usize, 8)?;

    let end = start.elapsed();

    write!(file, "Execution Time: {} seconds\t", end.as_secs())?;
    write!(file, "{} Primes Found\t", par_primes.len())?;
    writeln!(file, "Sum of Primes: {}", par_primes.iter().sum::<usize>())?;

    writeln!(file, "Top 10 Largest Primes:")?;
    for i in (1..=10).rev() {
        writeln!(file, "{}.\t {}", i, par_primes[par_primes.len() - i])?;
    }

    Ok(())
}

pub fn seq_prime_finder(n: usize) -> Result<Vec<usize>> {
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

    Ok(sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect())
}

pub fn par_prime_finder(n: usize, threads: usize) -> Result<Vec<usize>> {
    let nsq: usize = n.isqrt();
    
    let mut sieve = vec![true; nsq + 1];
    sieve[0] = false;
    sieve[1] = false;
    
    for i in 2..=nsq {
        if sieve[i] {
            let mut j = i.pow(2);
            while j <= nsq {
                sieve[j] = false;
                j = j + i;
            }
        }
    }

    let base_primes: Vec<usize> = (2..=nsq).filter(|&i| sieve[i]).collect();

    let chunk_size = (n + 1 - nsq) / threads + 1;
    let mut handles = Vec::new();

    for i in 0..threads {
        let base = base_primes.clone();
        let start = nsq + 1 + (i * chunk_size);
        let hand = thread::spawn(move || {
            let mut chunk = vec![true; chunk_size];
            for &p in &base {
                let mut multiple = if start <= p {
                    p * p
                } else {
                    let r = (start + p - 1) / p;
                    r * p
                };

                while multiple < start + chunk.len() {
                    if multiple >= start && multiple <= n {
                        chunk[multiple - start] = false;
                    }
                    multiple += p;
                }
            }
            chunk
        });
        handles.push(hand);
    }

    for hand in handles {
        let mut next_sieve = hand.join().map_err(|_| {
            Error::new(std::io::ErrorKind::Other, "thread panicked!")
        })?;
        sieve.append(&mut next_sieve);
    }
    sieve.truncate(n + 1);

    Ok(sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect())
}