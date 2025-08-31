import numpy as np
import pandas as pd

def execute():

    def calc_bayesian(priori, likelihood):
        weigths = priori * likelihood
        posteriori = weigths / sum(weigths)
        return(posteriori)

    prob_evento = float(input('Type the θ probability: '))
    priori = np.array([prob_evento, 1-prob_evento])
    
    likelihood_1 = float(input('Type P(Y=1/θ=1): '))
    likelihood_2 = float(input('Type P(Y=1/θ=0): '))
    likelihood = np.array([likelihood_1, likelihood_2])

    posteriori = calc_bayesian(priori, likelihood)

    data = {'priori': priori, 'posteriori': posteriori}

    probabilidades = pd.DataFrame(data
                                , index=['P(θ=1/Y=1)','P(θ=0/Y=1)'])
    return(probabilidades)