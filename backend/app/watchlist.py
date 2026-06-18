def demo():
    import sqlite3
    con = sqlite3.connect(':memory:')
    con.execute('create table quote(sym text, last real, pct real)')
    con.executemany('insert into quote values(?,?,?)', [
        ('600000', 10.2, 0.06), ('000001', 15.5, -0.04), ('300750', 200.1, 0.11)])
    con.commit()
    rule_pct = 0.05
    hits = con.execute('select sym, pct from quote where abs(pct)>?', (rule_pct,)).fetchall()
    return {'rule': f'|pct|>{rule_pct}', 'alerts': [{'sym': s, 'pct': p} for s, p in hits]}
