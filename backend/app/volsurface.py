def demo():
    import numpy as np
    strikes = np.array([90, 95, 100, 105, 110])
    iv = np.array([0.25, 0.22, 0.20, 0.21, 0.24])
    coef = np.polyfit(strikes, iv, 2)
    fit = np.polyval(coef, strikes)
    return {'strikes': strikes.tolist(), 'iv': iv.tolist(),
            'smile_fit': [round(float(x), 4) for x in fit]}
