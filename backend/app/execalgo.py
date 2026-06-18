def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(31)
    vol = pd.Series(rng.integers(1000, 5000, 8))
    target = 10000
    vwap_w = (vol / vol.sum() * target).round().astype(int)
    twap_w = pd.Series([target // 8] * 8)
    return {'vwap_slices': vwap_w.tolist(), 'twap_slices': twap_w.tolist(), 'target': target}
