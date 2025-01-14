k = input()
s = list(input())
t = list(input())

if len(s) < len(t):
    for i in range(len(s)):
        if s[i] != t[i]:
            s.insert(i, t[i])
            if s == t:
                print("Yes")
                break
            else:
                print("No")
                break
    else:
        s.append(t[len(s)])
        if s == t:
            print("Yes")
        else:
            print("No")
elif len(s) == len(t):
    if s == t:
        print("Yes")
    else:
        for i in range(len(s)):
            if s[i] != t[i]:
                s[i] = t[i]
                if s == t:
                    print("Yes")
                    break
                else:
                    print("No")
                    break
else:
    for i in range(len(t)):
        if s[i] != t[i]:
            s.pop(i)
            if s == t:
                print("Yes")
                break
            else:
                print("No")
                break
    else:
        t.append(s[len(t)])
        if s == t:
            print("Yes")
        else:
            print("No")
