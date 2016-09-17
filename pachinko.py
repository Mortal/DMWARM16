w, h, b, n = map(int, input().split())
board = [input().strip()[1:-1] for _ in range(h-1)]
buckets = input().strip()[1:-1]
input()  # Frame
bucket_scores = {k: int(v) for _ in range(b)
                 for k, v in [input().split()]}
queries = list(map(int, input().split()))

def fill(x):
    return [v if v is not None else (u + w) / 2
            for u, v, w in zip([x[1]]+x[:-1], x, x[1:]+[x[-2]])]

scores = []
# Last line
scores.append(
    fill([None if c == '|' else bucket_scores[c] for c in buckets]))
for row in reversed(board):
    scores.append(
        fill([None if c == '.' else scores[-1][i] for i, c in enumerate(row)]))
scores.reverse()
print(sum(scores[0][i-1] for i in queries))
