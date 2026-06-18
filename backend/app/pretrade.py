def demo():
    import sqlite3
    con = sqlite3.connect(':memory:')
    con.execute('create table limits(sym text, max_qty int)')
    con.executemany('insert into limits values(?,?)', [('600000', 5000), ('000001', 3000)])
    con.commit()
    order = {'sym': '600000', 'qty': 8000}
    cap = con.execute('select max_qty from limits where sym=?', (order['sym'],)).fetchone()[0]
    return {'order': order, 'limit': cap, 'rejected': order['qty'] > cap}
