w = 8
h = 78

while True:
    if w == h:
        print(f"Found t: {h}")
        break
    elif w < h:
        w += 101
    elif h < w:
        h += 103
