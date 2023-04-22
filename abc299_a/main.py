n = int(input())
s = input()

flag1 = False
flag2 = False
flag3 = False

for ss in s:
    if ss == '|' and flag2 == False:
        flag1 = True
    if ss == '*' and flag1 == True:
        flag2 = True
    if ss == '|' and flag1 == True and flag2 == True:
        flag3 = True

if flag1 == True and flag2 == True and flag3 == True:
    print("in")
else:
    print("out")
