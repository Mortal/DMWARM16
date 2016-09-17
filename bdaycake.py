n = int(input().strip())
counts = [0] * 720
for _ in range(n):
    dm = input().strip()
    i = (int(dm[2:]) - 1) * 30 + int(dm[:2]) - 1
    counts[i] += 1
    counts[i+360] += 1
bestdate = min(range(360),
               key=lambda md: sum(counts[i] for i in range(md, md + 30)))
m, d = divmod(bestdate, 30)
print('%02d%02d' % (d+1, m+1))
