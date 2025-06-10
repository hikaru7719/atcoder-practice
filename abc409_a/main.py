n = int(input())
t = list(input())
a = list(input())

for i in range(n):
    if t[i] == a[i] and t[i] == "o":
        print("Yes")
        exit()

print("No")
