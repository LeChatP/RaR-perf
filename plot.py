import json
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
import pandas as pd
import re
import sys

def format_as_float(x, _):
    return f"{x:.2f}"

# Load the JSON data
with open(sys.argv[1], 'r') as file:
    data = json.load(file)

# Extract relevant data
results = data["results"]
performance_data = []

regex = re.compile(r"bin/(sr|sudo) /usr/bin/true")

for result in results:
    parameters = result["parameters"]
    performance_data.append({
        "command": regex.sub(r'\1',result["command"]),
        "nb_roles": int(parameters["nb_roles"]),
        "nb_tasks": int(parameters["nb_tasks"]),
        "median": result["median"]* 1000,
        "mean": result["mean"]* 1000,
    })

# Convert to a DataFrame
df = pd.DataFrame(performance_data)

# Graph 1: Performance impact of `nb_roles`
# take only the values when nb_tasks=1
dfT = df[df["nb_tasks"] == 1]
plt.figure(figsize=(10, 6))

# Two lines: one for each command
statistic_type = "median"
plt.plot(dfT[dfT["command"] == "sudo"]["nb_roles"], dfT[dfT["command"] == "sudo"][statistic_type], label="sudo")
plt.plot(dfT[dfT["command"] == "sr"]["nb_roles"], dfT[dfT["command"] == "sr"][statistic_type], label="sr")
plt.title("Performance Impact of `nb_roles`")
plt.xlabel("Number of Roles")
plt.ylabel(f"{statistic_type.capitalize()} Performance")
plt.gca().yaxis.set_major_formatter(FuncFormatter(format_as_float)) 
plt.grid(True)
plt.legend()
plt.savefig(sys.argv[2])
plt.show()


# Graph 2: Performance impact of `nb_tasks`
# take only the values when nb_roles=1
dfR = df[df["nb_roles"] == 1]

plt.figure(figsize=(10, 6))
# Two lines: one for each command
plt.plot(dfR[dfR["command"] == "sudo"]["nb_tasks"], dfR[dfR["command"] == "sudo"][statistic_type], label="sudo")
plt.plot(dfR[dfR["command"] == "sr"]["nb_tasks"], dfR[dfR["command"] == "sr"][statistic_type], label="sr")
plt.title("Performance Impact of `nb_tasks`")
plt.xlabel("Number of Tasks")
plt.ylabel(f"{statistic_type.capitalize()} Performance (ms)")
plt.gca().yaxis.set_major_formatter(FuncFormatter(format_as_float)) 
plt.grid(True)
plt.legend()
plt.savefig(sys.argv[3])
plt.show()

# Graph 3: Performance impact of `nb_roles` and `nb_tasks`
plt.figure(figsize=(10, 6))
# Two lines: one for each command
plt.plot(df[df["command"] == "sudo"]["nb_tasks"]*df[df["command"] == "sudo"]["nb_roles"], df[df["command"] == "sudo"][statistic_type], label="sudo")
plt.plot(df[df["command"] == "sr"]["nb_tasks"]*df[df["command"] == "sr"]["nb_roles"], df[df["command"] == "sr"][statistic_type], label="sr")
plt.title("Performance Impact of `nb_roles` and `nb_tasks`")
plt.xlabel("Number of Tasks * Number of Roles")
plt.ylabel(f"{statistic_type.capitalize()} Performance (ms)")
plt.gca().yaxis.set_major_formatter(FuncFormatter(format_as_float)) 
plt.grid(True)
plt.legend()
plt.savefig(sys.argv[4])
plt.show()
