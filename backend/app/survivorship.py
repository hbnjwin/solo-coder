def demo():
    import sqlite3
    con = sqlite3.connect(':memory:')
    con.execute('create table listing(sym text, start text, end text)')
    con.executemany('insert into listing values(?,?,?)', [
        ('A', '2010-01-01', None), ('B', '2010-01-01', '2018-06-30'),
        ('C', '2015-03-01', None)])
    con.commit()
    asof = '2017-01-01'
    rows = con.execute(
        'select sym from listing where start<=? and (end is null or end>=?)',
        (asof, asof)).fetchall()
    return {'asof': asof, 'universe': [r[0] for r in rows]}
