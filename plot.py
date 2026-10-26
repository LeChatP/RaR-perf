"""Plot a hyperfine benchmark of the privilege tools, from its JSON export (hyperfine 1.x or 2.x).

    python3 plot.py <result.json[.gz]> <result.png>

writes <result.png>, the wall-clock time of each tool, <result>_overhead.png, the time above
/usr/bin/true run directly, <result>_memory.png, the peak resident memory, and <result>_fit.md,
the cost of one rule and the fixed cost of each tool. The time is the wall clock: run0 does its
work in polkitd and systemd, which the CPU time of the measured process does not count.
"""

import gzip
import json
import os
import statistics
import sys

import matplotlib

matplotlib.use("Agg")  # the benchmark runs without a display
import matplotlib.pyplot as plt
import numpy as np

BASELINE = "true"
# one color per tool, the same in every figure
COLORS = {
    "dosr": "tab:red",
    "sudo": "tab:blue",
    "sudo-rs": "tab:cyan",
    "doas": "tab:green",
    "please": "tab:purple",
    "run0": "tab:orange",
    BASELINE: "tab:gray",
}


# the colors of the versions of a tool that has none of its own
SPARE_COLORS = ["tab:brown", "tab:pink", "tab:olive", "black", "tab:red", "tab:cyan", "tab:purple"]
# the dashes of the versions of a tool, in the order they are met
VARIANT_LINES = [":", "-.", (0, (5, 2)), (0, (3, 1, 1, 1, 1, 1))]
_variants = {}


def style_of(tool):
    """The color and line of a tool: a version of a tool, named `<tool>-<version>` like
    `dosr-30cd12d`, has the color of the tool and dashes of its own."""
    if tool in COLORS:
        return COLORS[tool], "--" if tool == BASELINE else "-"
    base = tool.rsplit("-", 1)[0]
    if tool not in _variants:
        index = sum(1 for other in _variants if other.rsplit("-", 1)[0] == base)
        # a tool without a color of its own: one color per version, out of those of the tools
        color = COLORS.get(base) or SPARE_COLORS[index % len(SPARE_COLORS)]
        _variants[tool] = (color, VARIANT_LINES[index % len(VARIANT_LINES)])
    return _variants[tool]


def tool_of(command):
    """The tool of a command: its program, `true` for /usr/bin/true run directly."""
    return os.path.basename(command.split()[0])


def load(path):
    """{tool: {nb_tasks: (wall-clock times in ms, peak RSS in MiB)}}, from a JSON file, or one
    compressed by gzip, like the results of figures/"""
    with (gzip.open(path, "rt") if path.endswith(".gz") else open(path)) as file:
        data = json.load(file)
    series = {}
    for result in data["results"]:
        nb_tasks = result.get("parameters", {}).get("nb_tasks", 1)
        if isinstance(nb_tasks, dict):  # hyperfine 2: {"value": "10"}
            nb_tasks = nb_tasks["value"]
        nb_tasks = int(nb_tasks)
        if "measurements" in result:  # hyperfine 2: one entry per run, with its units
            runs = result["measurements"]
            assert all(run["time_wall_clock"]["unit"] == "second" for run in runs)
            times = [run["time_wall_clock"]["value"] * 1000 for run in runs]
            rss = [
                run["memory_peak_resident"]["value"] / 2**20
                for run in runs
                if "memory_peak_resident" in run
            ]
        else:  # hyperfine 1: lists in seconds and bytes
            times = [time * 1000 for time in result["times"]]
            rss = [memory / 2**20 for memory in result.get("memory_usage_byte", [])]
        series.setdefault(tool_of(result["command"]), {})[nb_tasks] = (times, rss)
    return series


def ordered(series):
    """The tools in the order of COLORS, the baseline last."""
    known = [tool for tool in COLORS if tool in series and tool != BASELINE]
    others = sorted(tool for tool in series if tool not in COLORS)
    return known + others


def quartiles(values):
    p25, p50, p75 = statistics.quantiles(values, n=4) if len(values) > 1 else values * 3
    return p25, statistics.median(values), p75


def plot_time(series, out):
    plt.figure(figsize=(10, 6))
    for tool in ordered(series) + ([BASELINE] if BASELINE in series else []):
        points = sorted(series[tool].items())
        n = [nb for nb, _ in points]
        q = [quartiles(times) for _, (times, _) in points]
        color, style = style_of(tool)
        label = "/usr/bin/true alone" if tool == BASELINE else tool
        plt.plot(n, [m for _, m, _ in q], linestyle=style, marker="o", markersize=3, label=label, color=color)
        plt.fill_between(n, [a for a, _, _ in q], [b for _, _, b in q], alpha=0.2, color=color)
    plt.xscale("log")
    plt.yscale("log")
    plt.title("Running /usr/bin/true through each tool, the last of N rules allowing it")
    plt.xlabel("Number of rules (N)")
    plt.ylabel("Wall-clock time (ms): median, band from p25 to p75")
    plt.grid(True, which="both", alpha=0.3)
    plt.legend()
    plt.savefig(out, dpi=150, bbox_inches="tight")
    plt.close()


def plot_overhead(series, out):
    if BASELINE not in series:
        return
    base = {nb: statistics.median(times) for nb, (times, _) in series[BASELINE].items()}
    plt.figure(figsize=(10, 6))
    for tool in ordered(series):
        points = [
            (nb, statistics.median(times) - base[nb])
            for nb, (times, _) in sorted(series[tool].items())
            if nb in base
        ]
        points = [(nb, value) for nb, value in points if value > 0]
        plt.plot(
            [nb for nb, _ in points],
            [value for _, value in points],
            linestyle=style_of(tool)[1],
            marker="o",
            markersize=3,
            label=tool,
            color=style_of(tool)[0],
        )
    plt.xscale("log")
    plt.yscale("log")
    plt.title("Time spent by each tool, above running /usr/bin/true directly")
    plt.xlabel("Number of rules (N)")
    plt.ylabel("Median overhead (ms)")
    plt.grid(True, which="both", alpha=0.3)
    plt.legend()
    plt.savefig(out, dpi=150, bbox_inches="tight")
    plt.close()


def plot_memory(series, out):
    plt.figure(figsize=(10, 6))
    for tool in ordered(series):
        points = [(nb, statistics.median(rss)) for nb, (_, rss) in sorted(series[tool].items()) if rss]
        plt.plot(
            [nb for nb, _ in points],
            [value for _, value in points],
            linestyle=style_of(tool)[1],
            marker="o",
            markersize=3,
            label=tool,
            color=style_of(tool)[0],
        )
    plt.xscale("log")
    plt.title("Peak resident memory of the measured process (polkitd and systemd not counted)")
    plt.xlabel("Number of rules (N)")
    plt.ylabel("Median peak RSS (MiB)")
    plt.grid(True, which="both", alpha=0.3)
    plt.legend()
    plt.savefig(out, dpi=150, bbox_inches="tight")
    plt.close()


def fit_table(series):
    """Markdown: time at the smallest N, cost of one rule from a least-squares line on the
    medians, and how well the line fits (R²)."""
    rows = [
        "| tool | median at N=min (ms) | median at N=max (ms) | cost of one rule (µs) | R² |",
        "|---|---|---|---|---|",
    ]
    for tool in ordered(series):
        points = sorted((nb, statistics.median(times)) for nb, (times, _) in series[tool].items())
        n = np.array([nb for nb, _ in points], dtype=float)
        median = np.array([value for _, value in points])
        if len(points) > 1:
            slope, intercept = np.polyfit(n, median, 1)
            predicted = slope * n + intercept
            total = ((median - median.mean()) ** 2).sum()
            r2 = 1 - ((median - predicted) ** 2).sum() / total if total else 1.0
            cost, quality = f"{slope * 1000:.2f}", f"{r2:.3f}"
        else:
            cost, quality = "-", "-"
        rows.append(
            f"| {tool} | {median[0]:.3f} (N={int(n[0])}) | {median[-1]:.3f} (N={int(n[-1])}) | {cost} | {quality} |"
        )
    return "\n".join(rows) + "\n"


def main():
    if len(sys.argv) != 3:
        print("Usage: python3 plot.py <result.json[.gz]> <result.png>")
        sys.exit(1)
    series = load(sys.argv[1])
    out = sys.argv[2]
    base, ext = os.path.splitext(out)
    plot_time(series, out)
    plot_overhead(series, f"{base}_overhead{ext}")
    plot_memory(series, f"{base}_memory{ext}")
    table = fit_table(series)
    with open(f"{base}_fit.md", "w") as file:
        file.write(table)
    print(table)


if __name__ == "__main__":
    main()
