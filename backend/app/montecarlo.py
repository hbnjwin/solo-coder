def demo():
    import numpy as np
    rng = np.random.default_rng(47)
    S0, K, T, r, sig, n = 100.0, 105.0, 1.0, 0.03, 0.2, 20000
    z = rng.standard_normal(n)
    ST = S0 * np.exp((r - sig ** 2 / 2) * T + sig * np.sqrt(T) * z)
    payoff = np.maximum(ST - K, 0)
    price = float(np.exp(-r * T) * payoff.mean())
    return {'mc_call_price': round(price, 4), 'paths': n,
            'std_err': round(float(payoff.std() / np.sqrt(n)), 5)}
