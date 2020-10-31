use primal::{Primes,is_prime};



fn main() {

    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();
    fn filter(p:usize,q:usize,lambda:(usize,usize))->bool{
        q>=p && lambda.1*q<=lambda.0*p
    }
    println!("       t;    m(S); max gen;    f(S);f/m");
    for t in 1..100000 {
        let lambda=(3,2);
        let mut input:Vec<usize> = Vec::new();
        if is_prime(4*t+1) && is_prime(4*t+3) && is_prime(6*t+1){
             let p:usize = 4*t as usize+1;
             input.extend(one_million_primes.iter().filter(|q|filter(p,**q,lambda)));
             let semigroup = frobeniusnumber::compute(&input);
             println!("{:8};{:8};{:8};{:8};{}",t, semigroup.m, input.iter().max().unwrap(), semigroup.f, semigroup.f as f64/ semigroup.m as f64);
        }
    }
}