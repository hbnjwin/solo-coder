import math

def chi_square_test(observed, expected):
    chi_square = 0
    for o, e in zip(observed, expected):
        if e > 0:
            chi_square += ((o - e) ** 2) / e
    return chi_square

def chi_square_cdf(x, df):
    if x <= 0 or df < 1:
        return 0.0
    return regularized_gamma(df / 2.0, x / 2.0)

def regularized_gamma(a, x):
    if x < 0 or a <= 0:
        return 0.0
    if x < a + 1:
        return regularized_gamma_series(a, x)
    else:
        return 1 - regularized_gamma_cf(a, x)

def regularized_gamma_series(a, x):
    max_iter = 100
    eps = 3e-12
    ap = a
    total = 1.0 / a
    delta = total
    for _ in range(max_iter):
        ap += 1
        delta *= x / ap
        total += delta
        if abs(delta) < abs(total) * eps:
            break
    return total * math.exp(-x + a * math.log(x) - log_gamma(a))

def regularized_gamma_cf(a, x):
    max_iter = 100
    eps = 3e-12
    b = x + 1.0 - a
    c = 1e30
    d = 1.0 / b
    h = d
    for i in range(1, max_iter + 1):
        an = -i * (i - a)
        b += 2.0
        d = an * d + b
        if abs(d) < 1e-30:
            d = 1e-30
        c = b + an / c
        if abs(c) < 1e-30:
            c = 1e-30
        d = 1.0 / d
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < eps:
            break
    return h * math.exp(-x + a * math.log(x) - log_gamma(a))

def log_gamma(x):
    coef = [
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179e-2,
        -0.5395239384953e-5
    ]
    y = x
    tmp = x + 5.5
    tmp -= (x + 0.5) * math.log(tmp)
    ser = 1.000000000190015
    for j in range(6):
        y += 1
        ser += coef[j] / y
    return -tmp + math.log(2.5066282746310005 * ser / x)

def chi_square_p_value(chi_square, df):
    return 1 - chi_square_cdf(chi_square, df)

def perform_ab_test_analysis(variant_data):
    if len(variant_data) < 2:
        return None
    
    total_success = sum(v['success_count'] for v in variant_data)
    total_calls = sum(v['call_count'] for v in variant_data)
    
    if total_calls == 0:
        return None
    
    overall_success_rate = total_success / total_calls
    
    observed = []
    expected = []
    for v in variant_data:
        observed.append(v['success_count'])
        observed.append(v['call_count'] - v['success_count'])
        expected_success = v['call_count'] * overall_success_rate
        expected.append(expected_success)
        expected.append(v['call_count'] - expected_success)
    
    chi_square = chi_square_test(observed, expected)
    df = (len(variant_data) - 1)
    p_value = chi_square_p_value(chi_square, df)
    
    return {
        'chi_square': chi_square,
        'degrees_of_freedom': df,
        'p_value': p_value,
        'is_significant': p_value < 0.05,
        'significance_level': 0.05
    }
