def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(13)
    w1 = pd.Series(rng.dirichlet(np.ones(8)), index=[f'S{i}' for i in range(8)])
    w2 = pd.Series(rng.dirichlet(np.ones(8)), index=w1.index)
    turnover = float((w2 - w1).abs().sum() / 2)
    return {'turnover': round(turnover, 4), 'names': list(w1.index)}
