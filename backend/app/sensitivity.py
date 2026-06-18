def demo():
    import numpy as np, pandas as pd
    fast = [5, 10, 15]
    slow = [20, 40, 60]
    rng = np.random.default_rng(71)
    grid = pd.DataFrame([[round(float(rng.normal(0.8, 0.3)), 3) for _ in slow] for _ in fast],
                        index=fast, columns=slow)
    return {'fast': fast, 'slow': slow, 'sharpe_grid': grid.values.tolist()}
