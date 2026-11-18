#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Writes Buzón's app icons from its portrait artwork (assets/app-icon/README.md).

    python3 tools/icon-svg.py assets/app-icon/buzon-art.png

The artwork is the 1024 px full-bleed portrait made by nodalia-design's tools/app-portrait. This
writes the 512 master `assets/app-icon/buzon.svg` (the tile, radius 112), the Linux icon on
Colloid's grid (a 224 tile at 16 on a 256 canvas, radius 51.5, soft shadow) as the hicolor
scalable SVG, its 256 and 512 PNGs, the Beta copies, and the 192 px tile shown on the website.
Needs Pillow.
"""
import base64
import io
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parent.parent
ICONS = ROOT / 'data' / 'icons'
APP = 'dev.danielmiguelt.Buzon'
COMMENT = ('<!-- Buzón app icon: Rowland Hill, from a photograph taken before 1879 (public '
           'domain), cut out in black and white on Buzón\'s red. '
           'AGPL-3.0-or-later (assets/app-icon/README.md). -->')


def encode(image, fmt, size=None, **kw):
    if size:
        image = image.resize((size, size), Image.LANCZOS)
    buffer = io.BytesIO()
    image.save(buffer, fmt, **kw)
    return base64.b64encode(buffer.getvalue()).decode()


def jpeg(art, size):
    return encode(art.convert('RGB'), 'JPEG', size, quality=90, optimize=True, progressive=True)


def shadow(scale):
    """Colloid's soft shadow 5 px below the tile, on the 256 canvas at `scale`."""
    canvas, margin, tile = 256, 16, 224
    mask = Image.new('L', (canvas * scale, canvas * scale), 0)
    top = (margin + 5) * scale
    ImageDraw.Draw(mask).rounded_rectangle(
        (margin * scale, top, (margin + tile) * scale, top + tile * scale),
        radius=51.5 * scale, fill=round(255 * 0.144))
    mask = mask.filter(ImageFilter.GaussianBlur(3.5 * scale))
    layer = Image.new('RGBA', mask.size, (0, 0, 0, 0))
    layer.putalpha(mask)
    return layer


def tile(art, size, radius):
    image = art.convert('RGBA').resize((size, size), Image.LANCZOS)
    mask = Image.new('L', (size * 4, size * 4), 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, size * 4 - 1, size * 4 - 1), radius * 4, fill=255)
    image.putalpha(mask.resize((size, size), Image.LANCZOS))
    return image


def linux_png(art, size):
    scale = size // 256
    canvas = shadow(scale)
    canvas.alpha_composite(tile(art, 224 * scale, 51.5 * scale), (16 * scale, 16 * scale))
    return canvas


def master_svg(art):
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512">\n'
        f'  {COMMENT}\n'
        '  <defs><clipPath id="tile"><rect width="512" height="512" rx="112"/></clipPath></defs>\n'
        f'  <image width="512" height="512" clip-path="url(#tile)" href="data:image/jpeg;base64,{jpeg(art, 1024)}"/>\n'
        '</svg>\n')


def linux_svg(art):
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">\n'
        f'  {COMMENT}\n'
        '  <defs><clipPath id="tile"><rect x="16" y="16" width="224" height="224" rx="51.5"/></clipPath></defs>\n'
        f'  <image width="256" height="256" href="data:image/png;base64,{encode(shadow(2), "PNG", optimize=True)}"/>\n'
        f'  <image x="16" y="16" width="224" height="224" clip-path="url(#tile)" href="data:image/jpeg;base64,{jpeg(art, 448)}"/>\n'
        '</svg>\n')


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    art = Image.open(sys.argv[1]).convert('RGB')
    (ROOT / 'assets' / 'app-icon' / 'buzon.svg').write_text(master_svg(art), encoding='utf-8')
    svg = linux_svg(art)
    for app in (APP, f'{APP}.Beta'):
        (ICONS / 'hicolor' / 'scalable' / 'apps' / f'{app}.svg').write_text(svg, encoding='utf-8')
        for size in (256, 512):
            path = ICONS / 'hicolor' / f'{size}x{size}' / 'apps' / f'{app}.png'
            linux_png(art, size).save(path, optimize=True)
    tile(art, 192, 42).save(ICONS / f'{APP}.png', optimize=True)


if __name__ == '__main__':
    main()
