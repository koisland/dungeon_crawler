
### Setup
Setup with pixi.
```bash
pixi install
```

### Split images
```bash
pixi run images
```

### Cleanup images
```bash
pixi run clean_images
```

### Maps
Into map
```bash
pixi run python data/map_to_tsv.py \
-t data/maps/map_test.tsv > data/maps/map_test.txt
```

Into TSV
```bash
pixi run python data/map_to_tsv.py \
-m data/maps/map_test.txt \
-c data/maps/map_test_cmap.tsv > data/maps/map_test.tsv
```
