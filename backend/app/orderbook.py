def demo():
    import numpy as np
    rng = np.random.default_rng(27)
    mid = 100.0
    bids = [(round(mid - 0.01 * i, 2), int(rng.integers(100, 1000))) for i in range(1, 6)]
    asks = [(round(mid + 0.01 * i, 2), int(rng.integers(100, 1000))) for i in range(1, 6)]
    bid_vol = sum(v for _, v in bids)
    ask_vol = sum(v for _, v in asks)
    return {'bids': bids, 'asks': asks,
            'imbalance': round((bid_vol - ask_vol) / (bid_vol + ask_vol), 3)}
