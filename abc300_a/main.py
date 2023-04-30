N, A, B = map(int, input().split())
x = A + B

list = list(map(int, input().split()))

for i,l in enumerate(list):
    if l == x:
        print(i+1)
