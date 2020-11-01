use primal::Primes;

fn main() {

    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();
    println!("1+lambda;       n;    m(S); max gen;    e(S);   #gaps;    #set;    f(S);f/m");

    // we want the specific example of p=48623
    for a in 5001..5002usize {
        let n=a+1; //makes p the n-th prime
        let mut b = a+500; // start with five hundred primes
        loop {
            let input = &one_million_primes[a..b];
            let ng = frobeniusnumber::compute(&input);
            let p = *input.first().unwrap();
            let q =*input.last().unwrap();
            let quot = q as f64/p as f64; // this is 1+lambda
            if ng.f < 20*p {
                // only show results for small quotients f/p
                println!("{:1.7}{:8};{:8};{:8};{:8};{:8};{:8};{:8};{}", quot, n, ng.m, ng.max_gen, ng.e, ng.count_gap, ng.count_set, ng.f, ng.f as f64 / ng.m as f64);
            }
            b=b+1;
            if q > ng.f { println!(); break; }
        }
    }
}