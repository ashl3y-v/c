import numpy as np
from scipy.constants import (
    G,
    N_A,
    c,
    convert_temperature,
    g,
    h,
    hbar,
    k,
)
from scipy.fft import (
    fft,
    fftfreq,
    fftn,
    fftshift,
    ifft,
    ifftn,
    ifftshift,
    irfft,
    irfftn,
    rfft,
    rfftfreq,
    rfftn,
)
from scipy.signal import (
    convolve,
    correlate,
)
from scipy.special import (
    bernoulli,
    beta,
    binom,
    comb,
    erf,
    factorial,
    factorial2,
    factorialk,
    gamma,
    lambertw,
    log_softmax,
    perm,
    rgamma,
    round,
    sinc,
    softmax,
    zeta,
)
from scipy.stats import (
    bernoulli,
    binom,
    binomtest,
    chi2_contingency,
    chisquare,
    cumfreq,
    describe,
    differential_entropy,
    ecdf,
    entropy,
    expon,
    fit,
    gamma,
    geom,
    goodness_of_fit,
    iqr,
    ks_1samp,
    ks_2samp,
    kstest,
    kurtosis,
    kurtosistest,
    linregress,
    mode,
    moment,
    monte_carlo_test,
    multinomial,
    multivariate_normal,
    multivariate_t,
    norm,
    normaltest,
    pearsonr,
    percentileofscore,
    poisson,
    randint,
    relfreq,
    scoreatpercentile,
    sem,
    skew,
    skewtest,
    t,
    trim1,
    trimboth,
    ttest_1samp,
    ttest_ind,
    ttest_ind_from_stats,
    ttest_rel,
    uniform,
    wasserstein_distance,
)

R = 0.082057366080960
