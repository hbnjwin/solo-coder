def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(37)
    px = pd.Series(100 * np.cumprod(1 + rng.normal(0, 0.015, 60)))
    entry = float(px.iloc[0])
    trail = 0.05
    peak = px.cummax()
    stop_line = peak * (1 - trail)
    hit = px[px < stop_line]
    return {'entry': round(entry, 2), 'trail_pct': trail,
            'stop_triggered': bool(len(hit) > 0),
            'exit_price': round(float(hit.iloc[0]), 2) if len(hit) else None}
