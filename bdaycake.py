from collections import Counter
n = int(input().strip())
bdays = [input() for _ in range(n)]
bdays = Counter((int(d[2:]) - 1) * 30 + int(d[:2]) - 1 for d in bdays)
dates = [m * 30 + d for m in range(12) for d in range(30)]
counts = [bdays.get(0, 0)]
for md in dates[1:]:
    counts.append(counts[-1] + bdays.get(md, 0))
best = float('inf')
bestdate = None
for md in dates:
    m, d = divmod(md, 30)
    if m == 11:
        miss = counts[d-1] + (counts[-1] - counts[md-1])
    else:
        miss = counts[md+30-1] - (counts[md-1] if md else 0)
    if miss < best:
        best = miss
        bestdate = md
m, d = divmod(bestdate, 30)
print('%02d%02d' % (d+1, m+1))
