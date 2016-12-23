with open("input.txt") as f:
    nodes = {}
    for line in f.read().strip().split('\n'):
        parts = line.split()
        loc = parts[0].split('-')
        nodes[(int(loc[1][1:]), int(loc[2][1:]))] = {
                "size" : int(parts[1][:-1]),
                "used" : int(parts[2][:-1]),
                "avail": int(parts[3][:-1]),
                "use%" : float(parts[4][:-1])/100
                }

    viable = 0
    for a_loc, a_info in nodes.items():
        for b_loc, b_info in nodes.items():
            if a_info["used"] < b_info["avail"] and \
               a_info["used"] > 0 and \
               a_loc != b_loc:
                viable += 1

    print(viable)
