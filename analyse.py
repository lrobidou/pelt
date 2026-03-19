import random as rd
import matplotlib.pyplot as plt
import subprocess
import json
from matplotlib.widgets import Slider


def flatten(xss: list[list[int]]):
    return [x for xs in xss for x in xs]


def get_interval_no_noise(nb_intervals: int) -> list[int]:
    interval_values = [rd.randint(0, 100) for _ in range(nb_intervals)]
    interval_lenghts = [rd.randint(10, 15) for _ in range(len(interval_values))]
    val = [
        [value] * length for (value, length) in zip(interval_values, interval_lenghts)
    ]
    val = flatten(val)
    return val


def make_some_noise(vals: list[int]) -> list[int]:
    return [max(val + rd.randint(-3, 3), 0) for val in vals]


def main():
    # rd.seed(0)

    penalties = [x / 10 for x in range(100)]

    val_origin = get_interval_no_noise(5)
    val = make_some_noise(val_origin)

    path = "./target/release/plot"
    # args = " ".join(str(x) for x in val)
    args = (
        [path]
        + [",".join([str(x) for x in val])]
        + [",".join([str(x) for x in penalties])]
    )
    print(" ".join(args))
    subprocess.run(args)
    with open("data.json", "r") as fichier:
        content = json.load(fichier)

    fig, ax = plt.subplots()
    plt.subplots_adjust(bottom=0.25)

    lines = []
    for data in content["data"]:
        penalty = data["penalty"]
        line = ax.vlines(data["breakpoints"], 0, 100, label=f"penalty {penalty}")
        line.set_visible(False)
        lines.append(line)

    les_x = range(len(val))
    ax.plot(les_x, val, "x", label="k-mer abundance")
    ax.plot(les_x, val_origin, "b--", label="ground truth")
    # for position in pelt_intervals:
    # plt.vlines(pelt_intervals, 0, 100, "red")

    # print(val)
    # plt.legend()
    # plt.xlabel("position in read")
    # plt.ylabel("abundance")
    # plt.show()

    # show initial version
    current = 0
    lines[current].set_visible(True)

    # slider axis
    ax_slider = plt.axes([0.2, 0.1, 0.6, 0.03])
    slider = Slider(ax_slider, "Version", 0, len(lines) - 1, valinit=current, valstep=1)

    def draw_legend():
        handles, labels = ax.get_legend_handles_labels()
        visible = [(h, l) for h, l in zip(handles, labels) if h.get_visible()]
        if visible:
            ax.legend(*zip(*visible))
        else:
            ax.legend([])

    def update(val):
        idx = int(slider.val)
        for i, line in enumerate(lines):
            line.set_visible(i == idx)

        # rebuild legend with only visible handles
        draw_legend()

        fig.canvas.draw_idle()

    slider.on_changed(update)

    # draw initial legend
    draw_legend()

    ax.set_xlabel("position in read")
    ax.set_ylabel("abundance")

    plt.show()


if __name__ == "__main__":
    main()
