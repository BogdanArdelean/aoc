import matplotlib.pyplot as plt

# Load coordinates from the uploaded file
file_path = "./input.txt"

points = []
with open(file_path, "r") as f:
    for line in f:
        line = line.strip()
        if line:
            x_str, y_str = line.split(",")
            points.append((int(x_str), int(y_str)))

# Separate X and Y
x = [p[0] for p in points]
y = [p[1] for p in points]

plt.figure(figsize=(8,8))
plt.plot(x, y, marker='o', markersize=2, linewidth=1)

# Label some of the points to avoid clutter (label every ~50th point)
for i, (px, py) in enumerate(points):
    # if i % 50 == 0:
    plt.text(px, py, f"({px},{py})", fontsize=6)

plt.title("Plot of Large Coordinate Path")
plt.grid(True)
plt.show()