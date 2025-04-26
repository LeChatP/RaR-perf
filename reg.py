import pandas as pd
from scipy.stats import linregress
import json
import sys
import re


with open(sys.argv[1], 'r') as avant_file:
    data = json.load(avant_file)

avant_results = data["results"]
avant_performance_data = []
statistic_type = "median"

regex = re.compile(r"bin/(sr|sudo) /usr/bin/true")

for result in avant_results:
    parameters = result["parameters"]
    avant_performance_data.append({
        "command": regex.sub(r'\1',result["command"]),
        "nb_roles": int(parameters["nb_roles"]),
        "nb_tasks": int(parameters["nb_tasks"]),
        "median": result["median"]* 1000,
        "mean": result["mean"]* 1000,
    })


with open(sys.argv[2], 'r') as apres_file:
    data_apres = json.load(apres_file)

apres_results = data_apres["results"]
apres_performance_data = []
for result in apres_results:
    parameters = result["parameters"]
    apres_performance_data.append({
        "command": regex.sub(r'\1',result["command"]),
        "nb_roles": int(parameters["nb_roles"]),
        "nb_tasks": int(parameters["nb_tasks"]),
        "median": result["median"]* 1000,
        "mean": result["mean"]* 1000,
    })

with open(sys.argv[3], 'r') as json_apres_file:
    json_data_apres = json.load(json_apres_file)

json_apres_results = json_data_apres["results"]
json_apres_performance_data = []
for result in json_apres_results:
    parameters = result["parameters"]
    json_apres_performance_data.append({
        "command": regex.sub(r'\1',result["command"]),
        "nb_roles": int(parameters["nb_roles"]),
        "nb_tasks": int(parameters["nb_tasks"]),
        "median": result["median"]* 1000,
        "mean": result["mean"]* 1000,
    })

avant_df = pd.DataFrame(avant_performance_data)
avant_dfR = avant_df[avant_df["nb_roles"] == 1]

apres_df = pd.DataFrame(apres_performance_data)
apres_dfR = apres_df[apres_df["nb_roles"] == 1]

json_apres_df = pd.DataFrame(json_apres_performance_data)
json_apres_dfR = json_apres_df[json_apres_df["nb_roles"] == 1]

avant_sr_line_x = avant_dfR[avant_dfR["command"] == "sr"]["nb_tasks"]
avant_sr_line_y = avant_dfR[avant_dfR["command"] == "sr"][statistic_type]

apres_sudo_line_x = apres_dfR[apres_dfR["command"] == "sudo"]["nb_tasks"]
apres_sudo_line_y = apres_dfR[apres_dfR["command"] == "sudo"][statistic_type]

apres_sr_line_x = apres_dfR[apres_dfR["command"] == "sr"]["nb_tasks"]
apres_sr_line_y = apres_dfR[apres_dfR["command"] == "sr"][statistic_type]

json_apres_sr_line_x = json_apres_dfR[json_apres_dfR["command"] == "sr"]["nb_tasks"]
json_apres_sr_line_y = json_apres_dfR[json_apres_dfR["command"] == "sr"][statistic_type]

avant_pente_sr, _, _, _, _ = linregress(avant_sr_line_x, avant_sr_line_y)
apres_pente_sudo, _, _, _, _ = linregress(apres_sudo_line_x, apres_sudo_line_y)
apres_pente_sr, _, _, _, _ = linregress(apres_sr_line_x, apres_sr_line_y)
json_apres_pente_sr, _, _, _, _ = linregress(json_apres_sr_line_x, json_apres_sr_line_y)

print(f"Pente avant optimisation : ms/task; sr={avant_pente_sr} ms/task")
print(f"Pente après optimisation : ms/task; sr={apres_pente_sr} ms/task")
print(f"Pente après optimisation (json) : ms/task; sr={json_apres_pente_sr} ms/task")
print(f"Pente sudo : ms/task; sudo={apres_pente_sudo} ms/task")

print(f"Réduction en pourcentage pour sr : {(avant_pente_sr - apres_pente_sr) / avant_pente_sr * 100:.2f}%")
print(f"Réduction entre sudo et sr après optimisation : {(apres_pente_sudo - apres_pente_sr) / apres_pente_sudo * 100:.2f}%")
print(f"Réduction entre sudo et sr avant optimisation : {(avant_pente_sr - apres_pente_sudo) / apres_pente_sr * 100:.2f}%")   
print(f"Réduction entre sudo et sr après optimisation (json) : {(json_apres_pente_sr - apres_pente_sudo) / json_apres_pente_sr * 100:.2f}%")
