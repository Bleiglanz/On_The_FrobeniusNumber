LoadPackage("NumericalSgps");

primes := Filtered([2..1000],IsPrime);

outfile := "../target/plot-some-lambda.csv";

PrintTo(outfile,"lambda;p;f_over_p\n");
for lambda in [0.1,0.5,1.0] do
    for p in primes do
        input := Filtered([p..p+Int(lambda*p)],IsPrime);
        if Length(input)>1 then
            ng := NumericalSemigroup(input);
            f := FrobeniusNumber(ng);
            AppendTo(outfile,lambda,";",p,";",1.0*f/p,"\n");
            Print(input,"\n");
        fi;
    od;
od;
Print("end");
# Plot for example
# dat = read.csv("../target/plot-some-lambda.csv",sep=";",header=TRUE,dec=".");
# ggplot(data=dat, aes(x=p, y=f_over_p, group=lambda,colour=lambda))+geom_line()+geom_point();

