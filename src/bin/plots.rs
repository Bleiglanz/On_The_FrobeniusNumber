use primal::Primes;

fn filter(p:usize,q:usize,lambda:(usize,usize))->bool{
    q>=p && lambda.1*q<=lambda.0*p
}

fn main() {
    let one_million_primes:Vec<usize> = Primes::all().take(1_000_000).collect();
    println!("lambda;p;f_over_p");
    for one_plus_lambda in &[(3,2),(21,10),(3,1)] {
        for p in one_million_primes.iter().take(1000){
            let mut input:Vec<usize> = Vec::new();
            input.extend(one_million_primes.iter().filter(|q|filter(*p,**q,*one_plus_lambda)));
            if input.len()>1 {
                let ng = frobeniusnumber::compute(&input);
                let lambda: f64 = one_plus_lambda.0 as f64 / one_plus_lambda.1 as f64 - 1.0;
                println!("{:1.1};{};{}", lambda, ng.m, (ng.f as f64 / ng.m as f64));
            }
        }
    }
}