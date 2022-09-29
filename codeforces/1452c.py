def test():
    squares, curves, moves = 0, 0, 0
    for c in input():
        if c == '(':
            curves += 1
        elif c == ')' and curves > 0:
            curves -= 1
            moves += 1
        elif c == '[':
            squares += 1
        elif c == ']' and squares > 0:
            squares -= 1
            moves += 1
    print(moves)

tc = int(input())
for t in range(tc):
    test()