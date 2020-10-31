use primal::Primes;
//
// just a simple main to compute the
//
fn main() {

    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();

    fn filter(p:usize,q:usize)->bool{
        q>=p && q<=2*p
    }
    println!("    e(S);    m(S); max gen;    f(S);f/m");
    for p in Primes::all().take(1000) {
        let mut input:Vec<usize> = Vec::new();
        input.extend(one_million_primes.iter().filter(|q|filter(p,**q)));
        let semigroup = frobeniusnumber::compute(&input);
        println!("{:8};{:8};{:8};{:8};{}", semigroup.e, semigroup.m, input.iter().max().unwrap(), semigroup.f, semigroup.f as f64/ semigroup.m as f64);
    }
}
