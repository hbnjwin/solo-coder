def demo():
    import numpy as np
    rng = np.random.default_rng(11)
    cov = np.cov(rng.normal(0, 0.02, size=(200, 3)), rowvar=False)
    w_mkt = np.array([0.4, 0.35, 0.25])
    delta = 2.5
    pi = delta * cov @ w_mkt
    return {'assets': ['stock', 'bond', 'commodity'],
            'implied_equilibrium_return': [round(float(x), 5) for x in pi],
            'market_weights': w_mkt.tolist()}
