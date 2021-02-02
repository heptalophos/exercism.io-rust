use std::ops::Range;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    
    if upper_bound > 1 { 
        sieve(2..(upper_bound + 1))
    } else {
        vec![]
    }
}

fn sieve(range: Range<u64>) -> Vec<u64> {

    let mut eratosthenes = range.collect::<Vec<u64>>();
    let mut primes = Vec::<u64>::new();
    
    while !eratosthenes.is_empty() { 
        let prime = eratosthenes[0];
        primes.push(prime);
        eratosthenes.retain(|n| n % prime != 0);
    }
    
    primes
}