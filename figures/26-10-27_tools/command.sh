#!/bin/sh
cd /home/lechatp/RootAsRole/perf
hyperfine -N --warmup 100 --runs 100 -L nb_tasks 1,2,7,21,59,166,464,1291,3593,10000 \
  --setup '/home/lechatp/RootAsRole/perf/bin/confgen {nb_tasks}' \
  --export-json /home/lechatp/RootAsRole/perf/results/26-10-27_08.19_tools/result.json \
  /usr/bin/true \
  '/home/lechatp/RootAsRole/perf/bin/dosr /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/sudo -n /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/doas -n /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/please -n /usr/bin/true'
