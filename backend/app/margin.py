def demo():
    import numpy as np
    positions = np.array([[100.0, 50, 0.5], [200.0, 30, 0.6]])
    notional = positions[:, 0] * positions[:, 1]
    margin = notional * positions[:, 2]
    return {'notional': notional.tolist(), 'margin_required': margin.tolist(),
            'leverage': round(float(notional.sum() / margin.sum()), 2)}
