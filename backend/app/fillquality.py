def demo():
    import numpy as np
    rng = np.random.default_rng(33)
    arrival = 100.0
    fills = arrival + rng.normal(0.01, 0.02, 40)
    slippage_bps = (fills - arrival) / arrival * 1e4
    return {'avg_slippage_bps': round(float(slippage_bps.mean()), 2),
            'worst_bps': round(float(slippage_bps.max()), 2), 'fills': int(len(fills))}
