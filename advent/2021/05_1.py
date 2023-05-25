import fileinput

def parse_points(line):
    u, v = line.rstrip().split(" -> ")
    ux, uy = u.split(",")
    vx, vy = v.split(",")
    return (int(ux), int(uy), int(vx), int(vy))

a = [parse_points(line) for line in fileinput.input()]

d = dict()

for ux, uy, vx, vy in a:
    if ux == vx:
        for y in range(min(uy, vy), max(uy, vy)+1):
            try:
                d[(ux, y)] += 1
            except KeyError:
                d[(ux, y)] = 1
    elif uy == vy:
        for x in range(min(ux, vx), max(ux, vx)+1):
            try:
                d[(x, uy)] += 1
            except KeyError:
                d[(x, uy)] = 1
    else:
            
print(len(list(filter(lambda p: p[1] >= 2, d.items()))))