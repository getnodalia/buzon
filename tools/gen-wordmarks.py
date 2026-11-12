#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Draw Buzón's wordmarks from Sora ExtraBold.

    python3 tools/gen-wordmarks.py PATH/TO/Sora-ExtraBold.ttf

The lettering is converted to outlines, so the app needs no font to show it.
Writes data/welcome/wordmark-{black,white}.png (1024 x 293, lettering alone)
and data/about/wordmark-{black,white}.svg (128 x 42, the app icon beside the
lettering). Needs fontTools, Pillow and cairosvg.
"""

import base64
import io
import sys
from pathlib import Path

import cairosvg
from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont
from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
ICON = ROOT / "data/icons/hicolor/256x256/apps/dev.danielmiguelt.Buzon.png"
TEXT = "Buzón"
INK = {"black": "#1b1b1f", "white": "#ffffff"}


def outline(font_path):
    """The word as one SVG path in font units (y down), and its bounds."""
    font = TTFont(font_path)
    glyphs = font.getGlyphSet()
    cmap = font.getBestCmap()
    path = SVGPathPen(glyphs)
    bounds = BoundsPen(glyphs)
    x = 0
    for ch in TEXT:
        name = cmap[ord(ch)]
        glyphs[name].draw(TransformPen(path, (1, 0, 0, -1, x, 0)))
        glyphs[name].draw(TransformPen(bounds, (1, 0, 0, -1, x, 0)))
        x += glyphs[name].width
    return path.getCommands(), bounds.bounds


def svg(d, bounds, width, height, ink, icon=None):
    x0, y0, x1, y1 = bounds
    pad = height * 0.08
    left = 0.0
    parts = []
    if icon is not None:
        side = height
        parts.append(
            f'<image width="{side}" height="{side}" '
            f'href="data:image/png;base64,{icon}"/>'
        )
        left = side + height * 0.18
    scale = min((height - 2 * pad) / (y1 - y0), (width - left - pad) / (x1 - x0))
    ty = (height - (y1 - y0) * scale) / 2 - y0 * scale
    tx = left - x0 * scale
    if icon is None:
        tx = (width - (x1 - x0) * scale) / 2 - x0 * scale
    parts.append(
        f'<path fill="{ink}" transform="translate({tx:.3f} {ty:.3f}) scale({scale:.6f})" d="{d}"/>'
    )
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}">' + "".join(parts) + "</svg>\n"
    )


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    d, bounds = outline(sys.argv[1])
    buf = io.BytesIO()
    Image.open(ICON).resize((84, 84), Image.LANCZOS).save(buf, "PNG", optimize=True)
    icon = base64.b64encode(buf.getvalue()).decode()
    for name, ink in INK.items():
        (ROOT / f"data/about/wordmark-{name}.svg").write_text(svg(d, bounds, 128, 42, ink, icon))
        plain = svg(d, bounds, 1024, 293, ink)
        cairosvg.svg2png(bytestring=plain.encode(), write_to=str(ROOT / f"data/welcome/wordmark-{name}.png"))


if __name__ == "__main__":
    main()
