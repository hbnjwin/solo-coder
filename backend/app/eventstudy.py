def demo():
    import numpy as np
    rng = np.random.default_rng(61)
    win = np.arange(-5, 6)
    ar = rng.normal(0, 0.005, len(win))
    ar[5:] += 0.01
    car = np.cumsum(ar)
    return {'window': win.tolist(), 'AAR': [round(float(x), 4) for x in ar],
            'CAR': [round(float(x), 4) for x in car]}
