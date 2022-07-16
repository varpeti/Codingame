import sys
import math

def parse_arg(arg):
    if arg == "_": return (None, None)
    if arg.startswith("$"): return ("ref", int(arg[1:]))
    return ("num", int(arg))

n = int(input())
cells = []
for i in range(n):
    operation, arg_1, arg_2 = input().split()
    c = {"op": operation, "a": parse_arg(arg_1), "b": parse_arg(arg_2)}
    cells.append(c)

def get_arg_value(arg) -> int:
    (t, e) = arg
    if t == "num": return e
    if t == "ref": return get_cell_value(cells[e])
    raise Exception()

def get_cell_value(c):
    if "val" in c: return c["val"]
    if c["op"] == "VALUE": c["val"] = get_arg_value(c["a"])                        ; return c["val"]
    if c["op"] == "ADD":   c["val"] = get_arg_value(c["a"]) + get_arg_value(c["b"]); return c["val"]
    if c["op"] == "SUB":   c["val"] = get_arg_value(c["a"]) - get_arg_value(c["b"]); return c["val"]
    if c["op"] == "MULT":  c["val"] = get_arg_value(c["a"]) * get_arg_value(c["b"]); return c["val"]
    raise Exception()

for i in range(n):
    print(get_cell_value(cells[i]))

