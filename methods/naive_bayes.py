from sklearn.preprocessing import OrdinalEncoder as OE
import numpy as np
import pandas as pd

encoder = OE()

def train_bayes(data, target, features):

    y_classes = data[target].unique()
    totais_y = data[target].value_counts()
    prior =  totais_y / len(data[target])

    

    print(y_classes)
    print(totais_y)
    print(prior)
