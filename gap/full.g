LoadPackage("NumericalSgps");
#
# computes the numerical semigroup generated by
# all primes greater than a given prime p
# shows the tendency of the quotient f/p to be 3
#
primes := Filtered([0..100000],IsPrime);

n := 0;
for p in primes do

    n := n+1; #makes p the n-th prime

    ng := NumericalSemigroup(Filtered([p..5*p],IsPrime));

    m:= Multiplicity(ng);

    f := FrobeniusNumber(ng);

    e := EmbeddingDimension(ng);

    num_gaps := GenusOfNumericalSemigroup(ng);

    num_set := 1+f - num_gaps;

    Print("n=",n,"   m=",m," e=",e," #gaps=",num_gaps," #set=",num_set," f=",f,"   f/m=",1.0*f/m,"\n");
od;

