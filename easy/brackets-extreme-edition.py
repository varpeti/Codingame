import sys
import math

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

expression = input()

# Write an answer using print
# To debug: print("Debug messages...", file=sys.stderr, flush=True)

stack = []

a = {'(',')','[',']','{','}'}

table = {'(':')','[':']','{':'}'}


ok = True
for i in expression:
    if i not in a: continue

    if i in table:
        stack.append(i)
    else:
        if len(stack)==0 or i != table[stack.pop(-1)]: 
            ok = False
            break

if ok and len(stack)==0: print("true")
else: print("false")

