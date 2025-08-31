from methods import bayes

method = True

while(method):
    method = int(input('Choose method: (1) Bayes; (2) GLM; (0) Exit: \n'))

    if method == 1:
        print(bayes.execute())
    elif method == 2:
        print('GLM')
    elif method != 0:
        print('Method invalid')