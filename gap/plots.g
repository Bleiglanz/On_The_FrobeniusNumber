LoadPackage("NumericalSgps");

primes := Filtered([2..3571],IsPrime);

outfile := "../target/plots_from_gap.csv";
PrintTo(outfile,"lambda;p;f_over_p\n");
for lambda in [0.5,1.1,2] do
    for p in primes do
        input := Filtered([p..p+Int(lambda*p)],IsPrime);
        if Length(input)>1 then
            ng := NumericalSemigroup(input);
            f := FrobeniusNumber(ng);
            AppendTo(outfile,lambda,";",p,";",1.0*f/p,"\n");
        fi;
    od;
od;