use primal::{Primes,is_prime};

//
// computes numerical semigroup created by the primes in the interval p..(3/2)p
// for primes in T(4), i.e. primes of the form p=4t+1 for which 4t+3 and 6t+1 are prime
//
fn main() {

    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();

    fn filter(p:usize,q:usize,one_plus_lambda:(usize,usize))->bool{
        q>=p && one_plus_lambda.1*q<=one_plus_lambda.0*p
    }

    println!("       t;    m(S); max gen;    f(S);f/m");
    for t in 1..100000 {
        let one_plus_lambda=(3,2);
        let mut input:Vec<usize> = Vec::new();
        if is_prime(4*t+1) && is_prime(4*t+3) && is_prime(6*t+1){
             let p:usize = 4*t as usize+1;
             input.extend(one_million_primes.iter().filter(|q|filter(p,**q,one_plus_lambda)));
             let ng = frobeniusnumber::compute(&input);
             assert_eq!(ng.max_gen,1+6*t as usize);
             println!("{:8};{:8};{:8};{:8};{}", t, ng.m, input.iter().max().unwrap(), ng.f, ng.f as f64/ ng.m as f64);
        }
    }
}