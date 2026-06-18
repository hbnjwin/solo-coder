def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(21)
    px = pd.DataFrame(100 * np.cumprod(1 + rng.normal(0.0003, 0.02, (120, 10)), axis=0),
                      columns=[f'S{i:02d}' for i in range(10)])
    mom = (px.iloc[-1] / px.iloc[-21] - 1).sort_values(ascending=False)
    return {'long': list(mom.head(3).index), 'short': list(mom.tail(3).index),
            'scores': mom.round(3).to_dict()}
