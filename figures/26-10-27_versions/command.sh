#!/bin/sh
cd /home/lechatp/RootAsRole/perf
hyperfine -N --warmup 100 --runs 100 -L nb_tasks 1,2,7,21,59,166,464,1291,3593,10000 \
  --setup '/home/lechatp/RootAsRole/perf/bin/genpol {nb_tasks} 1000 /home/lechatp/RootAsRole/perf/etc/versions v3.1.0:3.1.0:split v4.0.0:4.0.0:single v4.0.2:4.0.2:single v4.1.0:4.1.0:single dev:4.2.0:single' \
  --export-json /home/lechatp/RootAsRole/perf/results/26-10-27_08.31_versions/result.json \
  /usr/bin/true \
  '/home/lechatp/RootAsRole/perf/bin/rar-v3.1.0 /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/rar-v4.0.0 /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/rar-v4.0.2 /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/rar-v4.1.0 /usr/bin/true' \
  '/home/lechatp/RootAsRole/perf/bin/rar-dev /usr/bin/true'
