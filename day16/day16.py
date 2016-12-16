INPUT = "00111101111101000"

def checksum(init, n):
    data = list(map(int, init))
    while len(data) < n:
        data = data + [0] + list(map(lambda n: 1-n, reversed(data)))

    checksum = data[:n]
    while len(checksum) % 2 == 0:
        new = []
        for i in range(len(checksum)//2):
            if sum(checksum[2*i:2*i+2]) % 2 == 0:
                new.append(1)
            else:
                new.append(0)
        checksum = new
    return ''.join(map(str, checksum))


print("part 1: {}".format(checksum(INPUT, 272)))
print("part 2: {}".format(checksum(INPUT, 35651584)))
