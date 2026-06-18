def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(23)
    p = pd.Series(100 + np.cumsum(rng.normal(0, 0.5, 120)))
    win = 20
    ma = p.rolling(win).mean()
    sd = p.rolling(win).std()
    z = ((p - ma) / sd).dropna()
    return {'last_z': round(float(z.iloc[-1]), 3),
            'upper': round(float((ma + 2 * sd).iloc[-1]), 2),
            'lower': round(float((ma - 2 * sd).iloc[-1]), 2)}
