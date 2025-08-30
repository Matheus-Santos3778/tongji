import numpy as np

def calc_bayesian(priori, likelihood):
    weigths = priori * likelihood
    posteriori = weigths / sum(weigths)
    return(posteriori)

prob_evento = 0.65
priori = np.array([prob_evento, 1-prob_evento])
likelihood = np.array([0.9, 0.2])

print(priori)
print(calc_bayesian(priori, likelihood))