# SAFE-V

This is a project of the "Hands-On Dependability" course.
For more details, check out the [task sheet](https://hod.cs.uni-saarland.de/projects/P03.html).

docker run --rm --cpus=1 --stop-timeout 180 -v $PWD/src/fault_tree.rs:/opt/safe-v/src/fault_tree.rs -w /opt/safe-v hod cargo test -v
cargo run --release --bin bench