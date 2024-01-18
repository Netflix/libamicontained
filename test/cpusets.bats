load helpers

function setup() {
    setup_fake_rootfs
}

function teardown() {
    cleanup
}

@test "cpuset range" {
    echo 0-1 > "$TEMP_DIR/sys/fs/cgroup/cpuset.cpus.effective"
    check_num_cpus 2
}

@test "cpuset single" {
    echo 2 > "$TEMP_DIR/sys/fs/cgroup/cpuset.cpus.effective"
    check_num_cpus 1
}

@test "cpuset multiple" {
    echo 1,2 > "$TEMP_DIR/sys/fs/cgroup/cpuset.cpus.effective"
    check_num_cpus 2
}
