# Buzón app icon

**Figure:** Sir Rowland Hill (1795–1879). His 1837 pamphlet proposed a single cheap rate for a
letter anywhere in the country, paid in advance with a stamp, and the Penny Post of 1840 made it
so: the post as something everyone could send.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders,
the bust running off the bottom edge, in the Nodalia icon family (one historical figure per app,
one colour per app).

**Tile:** Buzón red, `#FA6B5D` at the top to `#D4241B` at the bottom; 512 × 512 viewBox, rounded
square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is a photograph of Hill taken before 1879 by an uncredited
photographer, printed as the frontispiece of the *Account of the Celebration of the Jubilee of
Uniform Inland Penny Postage* (London, General Post Office, 1891), on
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Rowland_Hill_photo_cleaned.jpg)
(public domain: anonymous work, `PD-UK-unknown`). It was cut out, converted to black and white and
placed over the app colour with nodalia-design's `tools/app-portrait`.

**Licence:** the icon made from it is licensed like the rest of Buzón, AGPL-3.0-or-later.

## Files

| File | What |
|---|---|
| `buzon-art.png` | the 1024 px artwork from `tools/app-portrait`, the source of every file below |
| `buzon.svg` | the 512 master: the artwork in the tile clip |
| `data/icons/hicolor/scalable/apps/dev.danielmiguelt.Buzon{,.Beta}.svg` | the Linux icon on Colloid's grid: a 224 tile at 16 on a 256 canvas, radius 51.5, soft shadow |
| `data/icons/hicolor/{256x256,512x512}/apps/…png` | its renders; the 512 one is also the window icon |
| `data/icons/dev.danielmiguelt.Buzon.png` | the 192 px tile shown on the website |

## Regenerate

```sh
# 1. the artwork, in nodalia-design (needs rembg):
python3 tools/app-portrait/portrait.py Rowland_Hill_photo_cleaned.jpg buzon-art.png \
  --box 417,230,1517,1330 --colors '#FA6B5D' '#D4241B'
# 2. every file above, here (needs Pillow):
python3 tools/icon-svg.py assets/app-icon/buzon-art.png
```
