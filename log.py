import numpy as np
import sys

# Configuration
if len(sys.argv) != 4:
    print("Usage: python log.py <start> <stop> <num_points>")
    sys.exit(1)

start = float(sys.argv[1])  # Début de la suite
stop = float(sys.argv[2])   # Fin de la suite
num_points = int(sys.argv[3])  # Nombre de points

# Générer une suite logarithmique
log_sequence = np.logspace(np.log10(start), np.log10(stop), num=num_points)

# Convertir en entiers
int_sequence = np.unique(log_sequence.astype(int))  # Supprime les doublons

# Générer une chaîne de nombres séparés par des virgules
result = ",".join(map(str, int_sequence))

print(result)
