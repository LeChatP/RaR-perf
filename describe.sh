#!/bin/sh
# describe.sh: the machine and the versions of the tools, that bench.yml writes in the env.txt of
# each result, since the times depend on them.
P=$(cd "$(dirname "$0")" && pwd)
echo "date: $(date -Iseconds)"
echo "kernel: $(uname -srm)"
echo "distribution: $(. /etc/os-release && echo "$PRETTY_NAME")"
echo "cpu: $(grep -m 1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //'), $(nproc) threads"
echo "frequency governor: $(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || echo none)"
echo "memory: $(awk '/MemTotal/ { printf "%.1f GiB", $2 / 1048576 }' /proc/meminfo)"
virt=$(systemd-detect-virt 2>/dev/null)
echo "virtualization: ${virt:-unknown}"
echo "hyperfine: $(hyperfine --version)"
# the commit of each tool built from its sources
for repo in RootAsRole sudo doas please "$P"/versions/*; do
    dir=$repo
    [ -d "$dir" ] || dir=$P/$repo
    [ -d "$dir/.git" ] || continue
    echo "${dir#"$P"/}: $(git -C "$dir" describe --tags --always --dirty) ($(git -C "$dir" rev-parse HEAD))"
done
echo "run0: $(run0 --version 2>/dev/null | head -n 1)"
echo "polkit: $(pkcheck --version 2>/dev/null)"
# sudo and dosr run their PAM stack even without a password
for service in sudo dosr; do
    echo "/etc/pam.d/$service: $(grep -v '^#' "/etc/pam.d/$service" 2>/dev/null | tr -s ' \t\n' ' ')"
done
