# RootAsRole performance test

> This README was written by Claude (an AI assistant) at the request of the maintainer, from the playbooks and scripts of this repository.


This repository measures how long a privilege tool takes to run a command, as its policy grows:
`dosr` of [RootAsRole](https://github.com/LeChatP/RootAsRole), sudo, doas, please and run0. It also
compares the versions of RootAsRole between them. The measures run with
[hyperfine](https://github.com/sharkdp/hyperfine), from Ansible playbooks.

> [!WARNING]
> The playbooks install setuid-root binaries, and binaries with file capabilities, that read
> policies the user of the benchmark can write. They also change files of `/etc` (see
> [What changes on the system](#what-changes-on-the-system)). Run them in a virtual machine.

## What is measured

Each tool runs `/usr/bin/true` as root, without a password, with a policy of N rules where only the
last rule allows `/usr/bin/true`. Before measuring a tool at a given N, hyperfine runs `confgen`
(`--setup`), which writes the policy of every tool with N rules:

| tool | policy | one rule |
|---|---|---|
| dosr | `etc/rootasrole.cbor` | one task of one role, with one command, run as uid 0 |
| sudo | `etc/sudoers` | one `Cmnd_Alias` of one command, listed as `(ALL) NOPASSWD:` in the line of the user (`#uid`) |
| doas | `etc/doas.conf` | `permit nopass <user> as root cmd <command>` |
| please | `/etc/please.ini` | one section, `require_pass = false` |
| run0 | `/etc/polkit-1/rules.d/10-run0.rules` | one `polkit.addRule` that allows `manage-units` to start `run-p*` units |

The commands of the other rules are `/usr/bin/true<n>`, which do not exist.

`/usr/bin/true` run alone is the baseline. `result_overhead.png` shows the time of each tool above
it.

Before the measure, `check.sh` asserts, at N=1 and at the largest N, that each tool runs
`/usr/bin/true` without a password and refuses `/usr/bin/id`. A measure would otherwise time a
refusal or a password prompt.

The tools do not do the same work. Keep this in mind when reading the plots:

- **PAM:**
  - sudo and dosr run their PAM account and session stacks, even without a password;
  - doas (`nopass`) and please (`require_pass = false`) do not call PAM.
- **run0:**
  - it asks polkitd and systemd to start the command: the time is the wall-clock time, and the
    memory plot only counts the process that hyperfine starts;
  - polkit never sees the command, so run0 cannot refuse `/usr/bin/id`;
  - polkitd reloads its rules when the file changes, so confgen does not rewrite a file that
    already holds the same policy.
- **please** reads only `/etc/please.ini`.
- **doas, please and run0** get the name of the user from confgen; sudo and dosr get its id.

## Requirements

- A virtual machine of the Arch, Debian, Fedora/RedHat or openSUSE family.
- Ansible, and cargo (with rustup or the package of the distribution).
- The sudo package of the distribution, which writes `/etc/pam.d/sudo`.
- systemd 256 or later for run0. `install.yml` installs polkit.

## Usage

```sh
ansible-playbook -K install.yml   # builds sudo, doas, please, RootAsRole and confgen
ansible-playbook bench.yml        # about 15 min; please at N=10000 takes most of it
```

A shorter run, of about 5 minutes:

```sh
ansible-playbook bench.yml -e bench_points=7 -e bench_runs=30 -e bench_warmup=10
```

To compare the versions of RootAsRole listed in `rar_versions`:

```sh
ansible-playbook -K versions.yml  # builds each version as bin/rar-<ref>, and genpol
ansible-playbook bench.yml -e compare=versions
```

Each version reads its own policy in `etc/versions/`. genpol writes it in the format of that
version: confgen only writes the format of the RootAsRole it is built with.

## Settings

`vars.yml` holds every setting:

- **Pinned versions:** `sudo_version`, `doas_version` and `please_version` are tags.
  `rar_version: dev` follows the branch: the commit of each run is in its `env.txt`, and a tag or a
  commit pins it.
- **RootAsRole settings:** RootAsRole keeps the build settings of its version. Only
  `rar_build_env` changes, and cargo gets it in the environment, which it prefers over the `[env]`
  of `.cargo/config.toml`:
  - the policy and the binaries are in this folder;
  - the policy is in CBOR, or in JSON with `rar_cfg_type`;
  - the policy is not immutable, since confgen rewrites it;
  - dosr does not authenticate.
- **Size of the measure:** `bench_max_tasks`, `bench_points`, `bench_warmup` and `bench_runs`. N
  goes from 1 to `bench_max_tasks`, with `bench_points` values evenly spaced on a log scale
  (`log.py`).
- **Commands:** `bench_tools` lists the commands to compare, each with its option to never ask a
  password.

## Results

Each run of `bench.yml` writes `results/<date>_<tools|versions>/`, which git ignores:

| file | content |
|---|---|
| `result.json` | the hyperfine export, every run of every command |
| `result.png` | the wall-clock time of each tool, median and band from p25 to p75, log-log |
| `result_overhead.png` | the median time above `/usr/bin/true` alone |
| `result_memory.png` | the median peak resident memory |
| `result_fit.md` | the median at the smallest and largest N, and the cost of one rule (least squares on the medians) |
| `command.sh` | the hyperfine command of the run, to measure again |
| `env.txt` | the machine, the commit of each tool, and the PAM stacks of sudo and dosr |

`python3 plot.py <result.json> <out.png>` plots a result again.

### Keeping a result

`keep.sh results/<run>/result.json <name>` copies a run into `figures/<name>/`, which the repository
holds:

- the hyperfine export, compressed (`result.json.gz`; `plot.py` reads it);
- the plots and `result_fit.md`;
- `command.sh` and `env.txt`.

Name it `<yy-mm-dd>_<what it shows>`, and keep only the runs that a document or a change cites.

### Kept results

The [Performance page](https://lechatp.github.io/RootAsRole/background/performance.html) of the
RootAsRole book shows these results.

- **2025, 3.1.0 in development** (`sr` was the name of dosr then). These runs used the scripts of
  that time, and only their plots are left:
  - `figures/25-04-16_sudo-sr-json/`: sudo and `sr` with a JSON policy, from 1 to 90 tasks;
  - `figures/25-04-16_sudo-sr-cbor/`: sudo and `sr` with a CBOR policy, from 1 to 400 tasks (the
    y axis is in seconds);
  - `figures/25-07-04_sudo-sr/`: sudo and `sr` after the optimisation of the policy engine, from 1
    to 10000 tasks.

## Files

| file | role |
|---|---|
| `vars.yml` | the settings |
| `install.yml` | builds and installs the tools into this folder |
| `versions.yml` | builds the versions of RootAsRole |
| `bench.yml` | checks the policies, measures, describes the machine and plots |
| `confgen/` | writes the policy of each tool with N rules (setuid root, mode 4750) |
| `genpol/` | writes the policy of each version of RootAsRole with N rules |
| `check.sh` | checks that each command allows `/usr/bin/true` and refuses `/usr/bin/id` |
| `describe.sh` | describes the machine and the versions of the tools |
| `plot.py` | plots a hyperfine export |
| `log.py` | the values of N |
| `keep.sh` | copies a run into `figures/` |

## What changes on the system

- **Packages:** `install.yml` installs the packages to build the tools and to plot.
- **System files:**
  - `/etc/pam.d/dosr`, written by `cargo xtask install` when it is missing;
  - `/etc/please.ini` and `/etc/polkit-1/rules.d/10-run0.rules`, rewritten by confgen before each
    measure;
  - the manual pages of RootAsRole, when they were built.
- **Privileged binaries in `bin/`:**
  - sudo, doas, please and confgen are setuid root;
  - dosr and the `rar-*` versions have file capabilities;
  - the policies they read are written by the user of the benchmark, through confgen or genpol;
  - confgen can write any file as root, so only the group of the user runs it.
