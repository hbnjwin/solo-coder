def demo():
    import numpy as np
    rng = np.random.default_rng(5)
    prices = 100 * np.cumprod(1 + rng.normal(0.0005, 0.01, 120))
    floor, m = 80.0, 3.0
    v = 100.0
    path = []
    for p_prev, p in zip(prices[:-1], prices[1:]):
        cushion = max(v - floor, 0)
        risky = min(m * cushion, v)
        v = v + risky * (p / p_prev - 1)
        path.append(round(v, 2))
    return {'floor': floor, 'multiplier': m, 'final_value': path[-1], 'nav_tail': path[-5:]}
