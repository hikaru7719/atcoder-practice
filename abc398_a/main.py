n = int(input())

if n % 2 == 0:
    x = (n - 2) // 2
    print("-" * x + "==" + "-" * x)
else:
    x = (n - 1) // 2
    print("-" * x + "=" + "-" * x)
