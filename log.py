import numpy as np
import sys

# Configuration
if len(sys.argv) != 4:
    print("Usage: python log.py <start> <stop> <num_points>")
    sys.exit(1)

start = float(sys.argv[1])  # first value
stop = float(sys.argv[2])   # last value
num_points = int(sys.argv[3])  # number of values

# values evenly spaced on a log scale
log_sequence = np.logspace(np.log10(start), np.log10(stop), num=num_points)

# as integers
int_sequence = np.unique(log_sequence.astype(int))  # without duplicates

# separated by commas, for hyperfine -L
result = ",".join(map(str, int_sequence))

print(result)
