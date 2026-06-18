def demo():
    import numpy as np, pandas as pd
    rng = np.random.default_rng(29)
    df = pd.DataFrame({'bid_sz': rng.integers(100, 900, 50), 'ask_sz': rng.integers(100, 900, 50)})
    df['ofi'] = (df['bid_sz'] - df['ask_sz']) / (df['bid_sz'] + df['ask_sz'])
    return {'mean_ofi': round(float(df['ofi'].mean()), 4),
            'last_ofi': round(float(df['ofi'].iloc[-1]), 4), 'n': len(df)}
