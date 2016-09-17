import sys
lines = iter(sys.stdin)
w, h, b, n = map(int, next(lines).split())
board = [next(lines).strip()[1:-1] for _ in range(h-1)]
buckets = next(lines).strip()[1:-1]
next(lines)  # Frame
bucket_scores = {k: int(v) for _ in range(b)
                 for k, v in [next(lines).split()]}
queries = list(map(int, next(lines).split()))
scores = []
# Last line
scores.append([None if c == '|' else bucket_scores[c] for c in buckets])
for i, v in enumerate(scores[-1]):
    if v is None:
        if i == 0:
            scores[-1][i] = scores[-1][i+1]
        elif i+1 == len(scores[-1]):
            scores[-1][i] = scores[-1][i-1]
        else:
            scores[-1][i] = (scores[-1][i-1] + scores[-1][i+1]) / 2
for row in reversed(board):
    scores.append([])
    for i, c in enumerate(row):
        if c == '.':
            if i == 0:
                scores[-1].append(scores[-2][i+1])
            elif i+1 == len(row):
                scores[-1].append(scores[-2][i-1])
            else:
                scores[-1].append((scores[-2][i-1] + scores[-2][i+1]) / 2)
        else:
            scores[-1].append(scores[-2][i])
scores.reverse()
print(sum(scores[0][i-1] for i in queries))
