use primal::Primes;
//
// just a simple main to compute the
//
fn main() {

    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();

    fn filter(p:usize,q:usize)->bool{
        q>=p && q<=2*p
    }

    let pi = |n|{one_million_primes.iter().filter(|x|{**x<=n}).count() };

    println!("    e(S);    m(S); max gen;sporadic;       f;   pi(p);   pi(2p)");
    for p in Primes::all().take(1000) {
        let mut input:Vec<usize> = Vec::new();
        input.extend(one_million_primes.iter().filter(|q|filter(p,**q)));
        let ng = frobeniusnumber::compute(&input);
        println!("{:8};{:8};{:8};{:8};{:8};{:8};{:8}", ng.e, ng.m, input.iter().max().unwrap(), ng.f, ng.count_set, pi(p), pi(2*p));
    }
}
