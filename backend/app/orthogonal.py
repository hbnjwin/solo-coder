def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(19)
    x = rng.normal(size=(100, 3))
    x[:, 1] += 0.8 * x[:, 0]
    df = pd.DataFrame(x, columns=['f1', 'f2', 'f3'])
    q, _ = np.linalg.qr(df.values)
    res = pd.DataFrame(q, columns=df.columns)
    return {'orig_corr': df.corr().round(2).values.tolist(),
            'orth_corr': res.corr().round(2).values.tolist()}
