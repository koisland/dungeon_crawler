import argparse


def main():
    ap = argparse.ArgumentParser()
    grp = ap.add_mutually_exclusive_group(required=True)
    grp.add_argument("-m", "--map", type=str, help="Map text file.")
    grp.add_argument("-t", "--tsv", type=str, help="Map TSV file.")

    ap.add_argument(
        "-c", "--char_key", type=str, required=False, help="Characters mapped to label."
    )

    args = ap.parse_args()
    mp: str | None = args.map
    tsv: str | None = args.tsv
    ckey: str | None = args.char_key

    char_to_label_map = {}
    if ckey:
        with open(ckey, "rt") as cfh:
            for line in cfh:
                c, lbl = line.strip().split("\t")
                char_to_label_map[c] = lbl
    if mp:
        assert char_to_label_map, "Character to label map must not be empty."
        with open(mp, "rt") as fh:
            for y, line in enumerate(fh):
                line = line.strip()
                for x, c in enumerate(line):
                    if c == " ":
                        continue
                    lbl = char_to_label_map[c]
                    print(x, y, lbl, c, sep="\t")
    elif tsv:
        xs, ys, lbls, cs = [], [], [], []
        with open(tsv, "rt") as fh:
            for line in fh:
                x, y, lbl, c = line.strip().split("\t")
                xs.append(int(x))
                ys.append(int(y))
                cs.append(c)
                lbls.append(lbl)

        max_x = max(xs)
        max_y = max(ys)
        tiles = {(x, y): (lbl, c) for x, y, lbl, c in zip(xs, ys, lbls, cs)}
        for y in range(max_y + 1):
            for x in range(max_x + 1):
                end = "\n" if x == max_x else ""
                _, c = tiles.get((x, y), (None, " "))
                print(c, sep="", end=end)


if __name__ == "__main__":
    raise SystemExit(main())
