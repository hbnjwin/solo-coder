def demo():
    import numpy as np
    rng = np.random.default_rng(7)
    rets = rng.normal(0.0008, 0.012, size=(250, 4))
    mu = rets.mean(axis=0)
    cov = np.cov(rets, rowvar=False)
    inv = np.linalg.pinv(cov)
    ones = np.ones(len(mu))
    w = inv @ ones / (ones @ inv @ ones)
    return {'assets': ['A', 'B', 'C', 'D'],
            'expected_return': [round(float(x), 5) for x in mu],
            'min_var_weights': [round(float(x), 4) for x in w]}
