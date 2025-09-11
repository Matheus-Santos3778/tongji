import numpy as np
import pandas as pd


def train_bayes(data, target, features):

    y_classes = data[target].unique()
    totais_y = data[target].value_counts()
    prior =  totais_y / len(data[target])

    total_y = data[target].value_counts().to_dict()

    print(y_classes)
    print(totais_y)
    print(prior)

    likelihoods = []
    for col in features:
        likelihood = data[[target, col]].value_counts().reset_index(name='count')
        
        likelihood['prob'] = likelihood.apply(
            lambda row: row['count'] / total_y[row[target]], axis=1
        )
    
        likelihoods.append(likelihood)

    print(likelihoods)
    