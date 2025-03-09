s = list(input())

stack = []

result = True
for i in range(len(s)):
    target = s[i]
    if s[i] == "(" or s[i] == "[" or s[i] == "<":
        stack.append(s[i])
    else:
        if len(stack) == 0:
            result = False
            break
        last = stack.pop(len(stack) - 1)
        if target == ")" and last == "(":
            continue
        elif target == "]" and last == "[":
            continue
        elif target == ">" and last == "<":
            continue
        else:
            result = False
            break
if len(stack) != 0:
    result = False

if result:
    print("Yes")
else:
    print("No")
