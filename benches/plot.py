#!/usr/bin/env python3
"""Draw the wfreq vs Unix pipeline benchmark: time and peak memory against
input size, one dot per run, as two side-by-side scatter panels.

    python3 benches/plot.py target/bench docs/images

Reads <bench>/wfreq.json and <bench>/pipeline.json (hyperfine --export-json,
written by benches/compare.sh), writes <out>/bench-light.svg and
<out>/bench-dark.svg, and prints the fastest runs as a Markdown table.
Standard library only.
"""
import json
import math
import os
import sys

SERIES = [("wfreq", "wfreq"), ("pipeline", "tr | sort | uniq -c | sort -rn")]
SANS = "system-ui, -apple-system, 'Segoe UI', Helvetica, Arial, sans-serif"
MONO = "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace"
# Chart tokens per theme. Series colors: categorical slots 1 (blue) and 2 (orange).
THEMES = {
    "light": dict(surface="#fcfcfb", edge="#0b0b0b", ink="#0b0b0b", ink2="#52514e",
                  grid="#e1e0d9", axis="#c3c2b7", series=["#2a78d6", "#eb6834"]),
    "dark": dict(surface="#1a1a19", edge="#ffffff", ink="#f0efec", ink2="#c3c2b7",
                 grid="#2c2c2a", axis="#383835", series=["#3987e5", "#d95926"]),
}
W, H = 960, 480
# fastest run: noise only ever adds time, so min is the steadiest estimate
best = min


def load(bench, name):
    """[(MB, [ms per run], [MiB per run])] for one hyperfine export, by size."""
    with open(os.path.join(bench, name + ".json")) as f:
        results = json.load(f)["results"]
    rows = []
    for r in results:
        n = r["parameters"]["n"]["value"]
        mb = os.path.getsize(os.path.join(bench, f"corpus-{n}.txt")) / 1e6
        ms = [m["time_wall_clock"]["value"] * 1e3 for m in r["measurements"]]
        mib = [m["memory_peak_resident"]["value"] / 2**20 for m in r["measurements"]]
        rows.append((mb, ms, mib))
    return sorted(rows)


def ticks(hi):
    """0 up to the first clean step (1, 2 or 5 x 10^k) at or above hi; at most 6 ticks."""
    base = 10 ** math.floor(math.log10(hi / 5))
    step = next(base * m for m in (1, 2, 5, 10) if hi / (base * m) <= 5)
    return [i * step for i in range(math.ceil(hi / step) + 1)]


def text(x, y, s, fill, size=14, anchor="start", extra=""):
    return (f'<text x="{x:.1f}" y="{y:.1f}" font-size="{size}" fill="{fill}" '
            f'text-anchor="{anchor}"{extra}>{s}</text>')


def panel(x0, title, unit, idx, data, t):
    """One scatter panel, 424 px wide at x0. idx 1 = time, 2 = memory."""
    top, bottom, left, right = 170, 404, x0 + 48, x0 + 424 - 80
    xs = ticks(max(r[0] for rows in data for r in rows))
    ys = ticks(max(v for rows in data for r in rows for v in r[idx]))
    sx = lambda v: left + (right - left) * v / xs[-1]
    sy = lambda v: bottom - (bottom - top) * v / ys[-1]
    tab = ' style="font-variant-numeric:tabular-nums"'

    out = [text(x0, 146, f"{title} ({unit})", t["ink"], 16, extra=' font-weight="600"')]
    for v in ys[1:]:
        out.append(f'<line x1="{left}" x2="{right}" y1="{sy(v):.1f}" y2="{sy(v):.1f}" stroke="{t["grid"]}"/>')
    out.append(f'<line x1="{left}" x2="{right}" y1="{bottom}" y2="{bottom}" stroke="{t["axis"]}"/>')
    out += [text(left - 10, sy(v) + 4.5, f"{v:,}", t["ink2"], anchor="end", extra=tab) for v in ys]
    out += [text(sx(v), bottom + 22, v, t["ink2"], anchor="middle", extra=tab) for v in xs]
    out.append(text((left + right) / 2, bottom + 46, "Input size (MB)", t["ink2"], anchor="middle"))

    # pipeline first so wfreq sits on top
    for color, rows in reversed(list(zip(t["series"], data))):
        pts = " ".join(f"{sx(r[0]):.1f},{sy(best(r[idx])):.1f}" for r in rows)
        out.append(f'<polyline points="{pts}" fill="none" stroke="{color}" stroke-width="2" '
                   'stroke-linejoin="round" stroke-linecap="round"/>')
        for r in rows:
            out += [f'<circle cx="{sx(r[0]):.1f}" cy="{sy(v):.1f}" r="4.5" fill="{color}" '
                    f'stroke="{t["surface"]}" stroke-width="2"/>' for v in r[idx]]
        # ponytail: end labels don't dodge each other; fine while the two series stay far apart
        end = best(rows[-1][idx])
        out.append(text(sx(rows[-1][0]) + 12, sy(end) + 4.5, f"{end:,.0f} {unit}", t["ink"],
                        extra=' font-weight="600"'))
    return out


def svg(t, data):
    (mb, wt, wm), (_, pt, pm) = data[0][-1], data[1][-1]
    lo = data[0][0][0]
    desc = (f"At {mb:.0f} MB, wfreq takes {best(wt):,.0f} ms and {best(wm):,.0f} MiB; "
            f"the pipeline takes {best(pt):,.0f} ms and {best(pm):,.0f} MiB.")
    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
        f'role="img" aria-labelledby="title desc" font-family="{SANS}">',
        '<title id="title">wfreq vs the Unix pipeline</title>',
        f'<desc id="desc">{desc}</desc>',
        f'<rect x="0.5" y="0.5" width="{W - 1}" height="{H - 1}" rx="12" fill="{t["surface"]}" '
        f'stroke="{t["edge"]}" stroke-opacity="0.1"/>',
        text(32, 46, "wfreq vs the Unix pipeline", t["ink"], 20, extra=' font-weight="600"'),
        text(32, 72, f"Top 10 words in {lo:.0f} to {mb:.0f} MB of English text. "
             "Each dot is one run, 10 runs per size. Lower is better.", t["ink2"], 15),
    ]
    for x, color, (_, label) in zip((32, 128), t["series"], SERIES):
        font = f' font-family="{MONO}"' if label != "wfreq" else ""
        out += [f'<line x1="{x}" x2="{x + 20}" y1="100" y2="100" stroke="{color}" stroke-width="2"/>',
                f'<circle cx="{x + 10}" cy="100" r="4.5" fill="{color}" stroke="{t["surface"]}" stroke-width="2"/>',
                text(x + 28, 105, label, t["ink"], 15, extra=font)]
    out += panel(32, "Time", "ms", 1, data, t)
    out += panel(504, "Peak memory", "MiB", 2, data, t)
    return "\n".join(out + ["</svg>", ""])


def main():
    assert ticks(700) == [0, 200, 400, 600, 800] and ticks(35) == [0, 10, 20, 30, 40]
    assert ticks(900) == [0, 200, 400, 600, 800, 1000] and ticks(60) == [0, 20, 40, 60]
    bench, out = sys.argv[1:3]
    data = [load(bench, key) for key, _ in SERIES]
    for name, t in THEMES.items():
        with open(os.path.join(out, f"bench-{name}.svg"), "w") as f:
            f.write(svg(t, data))

    print("| Input | wfreq | pipeline | wfreq memory | pipeline memory |")
    print("| ---: | ---: | ---: | ---: | ---: |")
    for (mb, wt, wm), (_, pt, pm) in zip(*data):
        print(f"| {mb:.0f} MB | {best(wt):,.0f} ms | {best(pt):,.0f} ms "
              f"| {best(wm):,.0f} MiB | {best(pm):,.0f} MiB |")
    (mb, wt, wm), (_, pt, pm) = data[0][-1], data[1][-1]
    print(f"\nAt {mb:.0f} MB: {best(pt) / best(wt):.1f}x faster, {best(pm) / best(wm):.0f}x less memory")


if __name__ == "__main__":
    main()
