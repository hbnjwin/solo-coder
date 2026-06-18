def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(59)
    idx = pd.date_range('2023-01-01', periods=400)
    ret = pd.Series(rng.normal(0.0003, 0.01, 400), index=idx)
    by_month = ret.groupby(ret.index.month).mean()
    return {'monthly_avg_return': {int(k): round(float(v), 5) for k, v in by_month.items()},
            'best_month': int(by_month.idxmax())}
