ROOT_DIR=$(git rev-parse --show-toplevel)

function make_cgroup()
{
    declare -g CGROUP_DIR=$(mktemp -p /sys/fs/cgroup -d libamicontained-test.XXXXXXXX)
}

function cleanup {
    rmdir "$CGROUP_DIR"
}

function check_num_cpus {
    expected_num_cpus=$1
    expected_recommended_threads=$2

    real_cgroup="/sys/fs/cgroup/tests.slice/$TEMP_NAME"
    output=$(sh -c 'echo $$ > '"$CGROUP_DIR/cgroup.procs; exec $ROOT_DIR/example")
    status=$?
    [ "$status" -eq 0 ]

    output=$(echo $output | tr -d '\n')
    echo "output: $output"

    # Construct the expected output string without newlines
    expected_output="num_cpus ${expected_num_cpus} recommended_threads ${expected_recommended_threads}"

    # Remove newlines from the actual output
    echo "expected output: $expected_output"

    [ "$output" == "$expected_output" ]
}
