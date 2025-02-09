a = list(map(int, input().split()))


if a[0] * a[1] == a[2]:
    print("Yes")
elif a[1] * a[2] == a[0]:
    print("Yes")
elif a[2] * a[0] == a[1]:
    print("Yes")
else:
    print("No")
