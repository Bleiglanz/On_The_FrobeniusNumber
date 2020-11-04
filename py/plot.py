import numpy as np
import matplotlib.pyplot as plt

plt.rcParams.update({
    "pgf.texsystem": "pdflatex",
    "pgf.preamble": [
         r"\usepackage[utf8x]{inputenc}",
         r"\usepackage[T1]{fontenc}",
         r"\usepackage{cmbright}",
         ]
})

input_file = "../target/plots_from_rust.csv"
ng_data = np.genfromtxt(input_file, delimiter=';', skip_header=1,
                        dtype=[('lambda', 'f8'), ('p', 'f8'), ('f_over_p', 'f8')])

lambdas = np.unique(ng_data['lambda'])

for lam in lambdas:
    print(lam)  # lambda is a keyword :-)
    filtered = ng_data['lambda'] == lam
    data = ng_data[filtered]
    p = data['p'][0]
    plt.tight_layout()
    plt.text(0.5, 3., "serif", family="serif")
    plt.text(0.5, 2., "monospace", family="monospace")
    plt.text(2.5, 2., "sans-serif", family="sans-serif")
    plt.xlim(right=4)

    fig = plt.figure()
    ax3 = fig.add_subplot(111)
    ax3.set_title("$\lambda={}$".format(lam))
    ax3.set_xlabel('$p$')
    ax3.set_ylabel("$f/p$".format(p))
    xdata = data['p']
    ydata = data['f_over_p']
    ax3.plot(xdata, ydata)
    plt.grid(color='grey', linestyle='-', linewidth=0.5)
    out = "../pdf/plot_f_over_p_lambda{0}.pdf".format(lam)
    plt.savefig(out, dpi=600)
