def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(53)
    sectors = ['finance', 'tech', 'consumer', 'pharma', 'energy']
    px = pd.DataFrame(100 * np.cumprod(1 + rng.normal(0.0004, 0.012, (60, 5)), axis=0), columns=sectors)
    rs = (px.iloc[-1] / px.iloc[-21] - 1).sort_values(ascending=False)
    return {'leaders': list(rs.head(2).index), 'laggards': list(rs.tail(2).index),
            'rs': rs.round(3).to_dict()}
