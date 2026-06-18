def demo():
    import numpy as np
    rng = np.random.default_rng(9)
    bench = rng.normal(0.0004, 0.009, 200)
    port = bench + rng.normal(0, 0.002, 200)
    te = float(np.std(port - bench) * np.sqrt(252))
    return {'tracking_error_annual': round(te, 4),
            'corr': round(float(np.corrcoef(port, bench)[0, 1]), 4)}
