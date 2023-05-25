import fileinput

total = 0

for line in fileinput.input():
    l, r = line.rstrip().split(" | ")

    cue = [""] * 10
    lp = [set(x) for x in l.split(' ')]
    rp = [set(x) for x in r.split(' ')]

    for s in lp:
        ln = len(s)
        if ln == 2:
            cue[1] = s
        elif ln == 3:
            cue[7] = s
        elif ln == 4:
            cue[4] = s
        elif ln == 7:
            cue[8] = s
        else:
            pass
        
    for s in lp:
        if len(s) == 6:
            if len(s & cue[1]) == 1:
                cue[6] = s
            elif len(s & cue[4]) == 3:
                cue[0] = s
            else:
                cue[9] = s
    
    for s in lp:
        if len(s) == 5:
            if len(s & cue[1]) == 2:
                cue[3] = s
            elif len(s & cue[6]) == 5:
                cue[5] = s
            else:
                cue[2] = s
    
    num = 0
    for o in rp:
        num *= 10
        for i in range(1, 10):
            if o == cue[i]:
                num += i

    total += num

print(total)