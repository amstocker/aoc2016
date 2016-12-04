from functools import reduce

with open("input.txt") as f:

    def update(info, instr):
        info[0] = (info[0] + (1 if instr[0] == 'R' else -1)) % 4
        info[(2,1,2,1)[info[0]]] += (1,1,-1,-1)[info[0]] * int(instr[1:])
        return info

    final = reduce(update, f.read().strip().split(", "), [0,0,0])

    print(abs(final[1]) + abs(final[2]))
