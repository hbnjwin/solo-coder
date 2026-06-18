def demo():
    import numpy as np
    from scipy.stats import norm
    S, K, T, r, sig = 100.0, 100.0, 0.5, 0.03, 0.2
    d1 = (np.log(S / K) + (r + sig ** 2 / 2) * T) / (sig * np.sqrt(T))
    d2 = d1 - sig * np.sqrt(T)
    call = S * norm.cdf(d1) - K * np.exp(-r * T) * norm.cdf(d2)
    return {'call_price': round(float(call), 4), 'delta': round(float(norm.cdf(d1)), 4),
            'gamma': round(float(norm.pdf(d1) / (S * sig * np.sqrt(T))), 5)}
