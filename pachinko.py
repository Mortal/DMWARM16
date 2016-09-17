w, h, b, n = map(int, input().split())
board = [input().strip()[1:-1] for _ in range(h-1)]
buckets = input().strip()[1:-1]
input()  # Frame
bucket_scores = {k: int(v) for _ in range(b)
                 for k, v in [input().split()]}
queries = list(map(int, input().split()))

def fill(values):
    for i, v in enumerate(values):
        if v is None:
            if i == 0:
                values[i] = values[i+1]
            elif i+1 == len(values):
                values[i] = values[i-1]
            else:
                values[i] = (values[i-1] + values[i+1]) / 2
    return values

scores = []
# Last line
scores.append(
    fill([None if c == '|' else bucket_scores[c] for c in buckets]))
for row in reversed(board):
    scores.append(
        fill([None if c == '.' else scores[-1][i] for i, c in enumerate(row)]))
scores.reverse()
print(sum(scores[0][i-1] for i in queries))
