import os
import re

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

df = pd.read_csv("benchmarks.csv")


def parse_time(s):
    if pd.isna(s) or not s.strip():
        return None
    m = re.match(r"([\d.]+)\s*(\w+)", s.strip())
    if not m:
        return None
    v, u = float(m.group(1)), m.group(2)
    if u == "ns":
        return v / 1000.0
    elif u == "µs" or u == "us":
        return v
    elif u == "ms":
        return v * 1000.0
    elif u == "s":
        return v * 1000_000.0
    else:
        raise Exception(f"unknown time unit {u}")


df["time_us"] = df["time"].apply(parse_time)
df = df.dropna(subset=["time_us"])

sns.set_theme(style="whitegrid")

for func in df["function"].unique():
    g = df[df["function"] == func].sort_values("size")
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 10))

    # Linear scale
    sns.lineplot(
        data=g,
        x="size",
        y="time_us",
        hue="implementation",
        marker="o",
        markersize=8,
        linewidth=2.5,
        palette="viridis",
        ax=ax1,
    )
    ax1.set_title(f"{func} (Linear)")
    ax1.set_xlabel("Size")
    ax1.set_ylabel("Time (µs)")
    ax1.grid(True, which="both", ls="-", alpha=0.2)

    # Logarithmic scale
    sns.lineplot(
        data=g,
        x="size",
        y="time_us",
        hue="implementation",
        marker="o",
        markersize=8,
        linewidth=2.5,
        palette="viridis",
        ax=ax2,
    )
    ax2.set_yscale("log")
    ax2.set_xscale("log")
    ax2.set_title(f"{func} (Logarithmic)")
    ax2.set_xlabel("Size")
    ax2.set_ylabel("Time (µs)")
    ax2.grid(True, which="both", ls="-", alpha=0.2)

    plt.tight_layout()
    plt.savefig(
        os.path.join("plots", f"benchmark_{func.replace(' ', '_')}.png"),
        dpi=300,
        bbox_inches="tight",
    )
    plt.close()

print("Done. Plots saved as benchmark_*.png")
