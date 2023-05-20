n = int(input())
s = input()
a = s.count('A')
t = s.count('T')
a2 = s.rfind('A')
t2 = s.rfind('T')

if t < a:
    print("A")
elif a < t:
    print("T")
else:
    if a2 < t2:
        print("A")
    else:
        print("T")
