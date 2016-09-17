w, h = map(int, input().split())
even = h // 2
odd = h - even
full = odd * (w // 2) + even * ((w - 1) // 2)
half = odd * (w % 2) + even * (1 + (w + 1) % 2)
print("%s %s" % (full, half))
