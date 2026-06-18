def demo():
    import sqlite3, pandas as pd
    con = sqlite3.connect(':memory:')
    internal = pd.DataFrame({'sym': ['A', 'B', 'C'], 'qty': [100, 200, 300]})
    broker = pd.DataFrame({'sym': ['A', 'B', 'C'], 'qty': [100, 180, 300]})
    internal.to_sql('internal', con, index=False)
    broker.to_sql('broker', con, index=False)
    m = internal.merge(broker, on='sym', suffixes=('_int', '_brk'))
    m['diff'] = m['qty_int'] - m['qty_brk']
    breaks = m[m['diff'] != 0]
    return {'total': int(len(m)), 'breaks': breaks.to_dict('records')}
