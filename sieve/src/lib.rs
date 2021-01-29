use std::ops::Range;
// use Sieve::primes_up_to;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 { 
        vec![] 
    } else {
        sieve(2..(upper_bound + 1))
    }
}

fn sieve(range: Range<u64>) -> Vec<u64> {
    eratosthenes(range.collect(), vec![])
}

fn eratosthenes(mut nrange: Vec<u64>, 
                mut primes: Vec<u64>) -> Vec<u64> {
    if nrange.is_empty() { primes }
    else { 
        let p = nrange[0];
        primes.push(p);
        nrange.retain(|&n| n % p != 0 );
        eratosthenes(nrange, primes)
    }
}