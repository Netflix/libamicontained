ROOT_DIR=$(git rev-parse --show-toplevel)

function make_tempdir()
{
    declare -g TEMP_DIR=$(mktemp -p $(pwd) -d huldufolk-test.XXXXXXXX)
}

function cleanup {
    rm -rf "$TEMP_DIR"
}

function setup_fake_rootfs {
    make_tempdir

    mkdir -p "$TEMP_DIR/sys/fs/cgroup/" "$TEMP_DIR/proc/self"
    echo "0::/" > "$TEMP_DIR/proc/self/cgroup"
    echo 0-1 > "$TEMP_DIR/sys/fs/cgroup/cpuset.cpus.effective"
    ln "$ROOT_DIR/example" "$TEMP_DIR/example"
}

function check_num_cpus {
    run unshare -Urm --root="$TEMP_DIR" /example
    echo $output
    [ "$status" -eq 0 ]
}
