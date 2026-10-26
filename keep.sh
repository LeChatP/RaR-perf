#!/bin/sh
# keep.sh <result.json> <name>: keeps a result of results/, which git ignores, in figures/<name>/,
# which the repository holds: its hyperfine results compressed, its plots, the cost of one rule
# and, when the run wrote them, the command that measured it and the description of the machine.
# python3 plot.py figures/<name>/result.json.gz <out.png> plots it again.
set -e
if [ $# -ne 2 ]; then
    echo "usage: $0 <result.json> <name>" >&2
    exit 1
fi
P=$(cd "$(dirname "$0")" && pwd)
json=$1
base=${json%.json}
run=$(dirname "$json")
dest=$P/figures/$2
if [ -e "$dest" ]; then
    echo "$dest already exists" >&2
    exit 1
fi
mkdir -p "$dest"
gzip -9c "$json" > "$dest/result.json.gz"
# the plots of plot.py are named after the JSON file
for suffix in .png _overhead.png _memory.png _fit.md; do
    if [ -f "$base$suffix" ]; then cp "$base$suffix" "$dest/result$suffix"; fi
done
for file in env.txt command.sh; do
    if [ -f "$run/$file" ]; then cp "$run/$file" "$dest/"; fi
done
ls "$dest"
