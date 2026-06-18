def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(3)
    df = pd.DataFrame({'value': rng.normal(size=20), 'quality': rng.normal(size=20),
                       'lowvol': rng.normal(size=20)}, index=[f'S{i:02d}' for i in range(20)])
    z = (df - df.mean()) / df.std()
    score = z.mean(axis=1)
    top = score.sort_values(ascending=False).head(5)
    return {'factors': list(df.columns), 'top_picks': top.round(3).to_dict()}
