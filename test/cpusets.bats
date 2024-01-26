load helpers

function setup() {
    make_cgroup
}

function teardown() {
    cleanup
}

@test "cpuset range" {
    echo 0-1 > "$CGROUP_DIR/cpuset.cpus"
    check_num_cpus 2 2
}

@test "cpuset single" {
    echo 2 > "$CGROUP_DIR/cpuset.cpus"
    check_num_cpus 1 1
}

@test "cpuset multiple" {
    echo 1,2 > "$CGROUP_DIR/cpuset.cpus"
    check_num_cpus 2 2
}

@test "cpu.max set" {
    echo 0-1 > "$CGROUP_DIR/cpuset.cpus"
    echo 100000 100000 > "$CGROUP_DIR/cpu.max"
    check_num_cpus 2 1
}

@test "cpu.max unset" {
    echo 0-1 > "$CGROUP_DIR/cpuset.cpus"
    echo max 100000 > "$CGROUP_DIR/cpu.max"
    check_num_cpus 2 2
}
