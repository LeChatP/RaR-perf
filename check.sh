#!/bin/sh
# check.sh <command>...: with the policies written last (by confgen or genpol), each command, a
# tool and its options, must run /usr/bin/true without a password, and refuse /usr/bin/id, which no
# rule allows; otherwise a measure would time a refusal or a password prompt. run0 cannot refuse
# it: polkit never sees the command.
FAILED=0
for tool in "$@"; do
    name=$(basename "${tool%% *}")
    expected_id=refused
    [ "$name" = run0 ] && expected_id=allowed
    # $tool unquoted: the program, then its options
    if $tool /usr/bin/true </dev/null >/dev/null 2>&1; then true_ok=yes; else true_ok=no; FAILED=1; fi
    if $tool /usr/bin/id -u </dev/null >/dev/null 2>&1; then id=allowed; else id=refused; fi
    [ "$id" = "$expected_id" ] || FAILED=1
    echo "$name: /usr/bin/true allowed: $true_ok, /usr/bin/id $id (expected $expected_id)"
done
exit $FAILED
