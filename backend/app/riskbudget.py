def demo():
    import numpy as np
    rng = np.random.default_rng(41)
    cov = np.cov(rng.normal(0, 0.02, (200, 3)), rowvar=False)
    w = np.array([1 / 3, 1 / 3, 1 / 3])
    port_var = w @ cov @ w
    mctr = cov @ w / np.sqrt(port_var)
    rc = w * mctr
    return {'risk_contribution': [round(float(x), 5) for x in rc],
            'pct': [round(float(x / rc.sum()), 3) for x in rc]}
