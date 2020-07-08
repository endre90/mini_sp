# gripper_inc_instances commands and results

In gripper_inc_instances, there are no variable groups and only the bare incremental algorithm is used.

### Results:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_inc 50 1` | 43.4 ± 2.4 | 39.3 | 48.6 | 1.00 |
| `cargo run --release --example gripper_inc 50 2` | 46.6 ± 1.4 | 44.3 | 49.9 | 1.08 ± 0.07 |
| `cargo run --release --example gripper_inc 50 3` | 199.7 ± 1.8 | 197.1 | 203.8 | 4.60 ± 0.25 |
| `cargo run --release --example gripper_inc 50 4` | 1017.2 ± 6.5 | 1012.6 | 1021.8 | 23.45 ± 1.28 |
| `cargo run --release --example gripper_inc 50 5` | 13505.2 ± 285.5 | 13303.4 | 13707.1 | 311.35 ± 18.14 |
| `cargo run --release --example gripper_inc 50 6` | 67508.7 ± 1608.0 | 66371.6 | 68645.7 | 1556.35 ± 92.28 |
| `cargo run --release --example gripper_inc 50 7` | 722988.0 ± 6296.0 | 718536.1 | 727440.0 | 16667.78 ± 916.59 |
| `cargo run --release --example gripper_inc 50 8` | timeout | - | - | - |