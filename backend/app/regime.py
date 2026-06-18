def demo():
    import numpy as np
    from sklearn.cluster import KMeans
    rng = np.random.default_rng(67)
    vol = np.concatenate([rng.normal(0.01, 0.002, 80), rng.normal(0.03, 0.004, 80)]).reshape(-1, 1)
    km = KMeans(n_clusters=2, n_init=10, random_state=0).fit(vol)
    return {'regime_counts': np.bincount(km.labels_).tolist(),
            'centers': sorted(round(float(c[0]), 4) for c in km.cluster_centers_)}
