def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(43)
    p = pd.Series(100 * np.cumprod(1 + rng.normal(0, 0.01, 100)))
    p.iloc[40] = np.nan
    p.iloc[70] = p.iloc[69] * 1.5
    ret = p.pct_change()
    outliers = ret[ret.abs() > 0.2]
    return {'missing': int(p.isna().sum()), 'outliers': int(len(outliers)), 'rows': int(len(p))}
