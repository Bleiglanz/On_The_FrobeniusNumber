use primal::Primes;

fn main() {

    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();
    fn filter(p:usize,q:usize)->bool{
        // we know from older experiments that we never use more primes in the range we consider
        q>=p && q<=5*p
    }
    println!("       n;    m(S); max gen;    e(S);   #gaps;    #set;    f(S);f/m");
    let mut n = 0; // simple counter
    for p in one_million_primes.iter().take(1000) {
        n = n+1; // p=p_n the n-th prime
        let mut input:Vec<usize> = Vec::new();
        input.extend(one_million_primes.iter().filter(|q|filter(*p,**q)));
        let ng = frobeniusnumber::compute(&input);
        assert!(ng.max_gen < 4*p);
        assert_eq!(ng.count_set+ng.count_gap,ng.f+1);
        println!("{:8};{:8};{:8};{:8};{:8};{:8};{:8};{}", n, ng.m, ng.max_gen, ng.e, ng.count_gap, ng.count_set, ng.f, ng.f as f64/ ng.m as f64);
    }
}