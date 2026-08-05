import os
import glob
from PIL import Image


SPRITE_SIZE = 32


def get_name(line: str) -> str | None:
    line = line.strip()
    if not line:
        return
    try:
        _, _, name = line.split(".")
    except ValueError:
        _, name = line.split(".")

    return (
        name.strip()
        .replace(" ", "_")
        .replace("(", "")
        .replace(")", "")
        .replace("/", "")
    )


def extract_tiles(img: Image.Image, key_path: str, outdir: str) -> None:
    os.makedirs(outdir, exist_ok=True)
    with open(file=key_path) as fh:
        row = 0
        idx = 0
        for line in fh:
            name = get_name(line)
            if not name:
                row += 1
                idx = 0
                continue

            outfile = os.path.join(outdir, f"{name}.png")

            # left, upper, right, lower
            left = idx * SPRITE_SIZE
            if name == "two_tile_tree":
                upper = (row - 1) * SPRITE_SIZE
            else:
                upper = row * SPRITE_SIZE
            right = (idx + 1) * SPRITE_SIZE
            lower = (row + 1) * SPRITE_SIZE

            # print(name, idx, row, (left, upper, right, lower))
            sprite = img.crop((left, upper, right, lower))
            with open(outfile, "wb") as ofh:
                sprite.save(ofh)
            idx += 1


def extract_animations(img: Image.Image, key_path: str, outdir: str):
    w = img.width
    os.makedirs(outdir, exist_ok=True)
    with open(file=key_path) as fh:
        for row, line in enumerate(fh):
            name = get_name(line)
            if not name:
                continue

            upper = row * SPRITE_SIZE
            lower = (row + 1) * SPRITE_SIZE

            # iter thru all sprite offsets and check if blank
            for i, offset in enumerate(range(0, w, SPRITE_SIZE)):
                left = offset
                right = offset + SPRITE_SIZE

                # print(name, idx, row, (left, upper, right, lower))
                sprite = img.crop((left, upper, right, lower))
                entropy = sprite.entropy()
                # Empty image
                if entropy == 0:
                    break
                outfile = os.path.join(outdir, f"{name}_{i}.png")
                with open(outfile, "wb") as ofh:
                    sprite.save(ofh)


def main():
    image_paths = glob.glob("data/*.png")
    for image_path in image_paths:
        image_type, _ = os.path.splitext(image_path)
        key_path = f"{image_type}.txt"
        if not os.path.exists(key_path):
            continue
        img = Image.open(image_path)
        is_animation = "animated-tiles" in image_type
        outdir = f"{image_type}_tiles"
        if is_animation:
            extract_animations(img, key_path, outdir)
        else:
            extract_tiles(img, key_path, outdir)


if __name__ == "__main__":
    raise SystemExit(main())
