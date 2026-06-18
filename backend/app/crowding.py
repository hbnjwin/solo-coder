def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(17)
    f = pd.DataFrame(rng.normal(size=(150, 4)), columns=['mom', 'rev', 'value', 'growth'])
    c = f.corr().abs()
    crowd = float((c.values.sum() - 4) / (16 - 4))
    return {'avg_pairwise_corr': round(crowd, 4), 'matrix': c.round(2).values.tolist(),
            'factors': list(f.columns)}
