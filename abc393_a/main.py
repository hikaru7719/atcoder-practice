a, b = map(str, input().split())

if a == "fine" and b == "fine":
    print("4")
elif a == "fine" and b == "sick":
    print("3")
elif a == "sick" and b == "fine":
    print("2")
elif a == "sick" and b == "sick":
    print("1")
