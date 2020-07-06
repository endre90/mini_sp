| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_inc 20 1` | 39.4 ± 1.3 | 37.3 | 42.7 | 1.00 |
| `cargo run --release --example gripper_inc 20 2` | 45.6 ± 1.7 | 42.6 | 52.1 | 1.16 ± 0.06 |
| `cargo run --release --example gripper_inc 20 3` | 198.9 ± 1.5 | 196.1 | 201.3 | 5.04 ± 0.16 |
| `cargo run --release --example gripper_inc 20 4` | 1029.9 ± 9.0 | 1016.8 | 1044.2 | 26.11 ± 0.86 |
| `cargo run --release --example gripper_inc 25 5` | 14994.4 ± 191.4 | 14725.5 | 15201.1 | 380.17 ± 13.02 |
| `cargo run --release --example gripper_inc 30 6` | 72487.0 ± 640.8 | 71371.5 | 73512.4 | 1837.83 ± 60.65 |
| `cargo run --release --example gripper_comp 20 1 g b r` | 61.7 ± 2.1 | 59.3 | 67.7 | 1.56 ± 0.07 |
| `cargo run --release --example gripper_comp 20 2 g b r` | 79.4 ± 1.9 | 77.4 | 85.4 | 2.01 ± 0.08 |
| `cargo run --release --example gripper_comp 20 3 g b r` | 118.0 ± 2.9 | 113.5 | 124.4 | 2.99 ± 0.12 |
| `cargo run --release --example gripper_comp 20 4 g b r` | 234.9 ± 3.1 | 228.4 | 239.4 | 5.96 ± 0.20 |
| `cargo run --release --example gripper_comp 25 5 g b r` | 998.3 ± 8.6 | 981.2 | 1009.9 | 25.31 ± 0.83 |
| `cargo run --release --example gripper_comp 30 6 g b r` | 12122.2 ± 64.3 | 12026.1 | 12216.5 | 307.35 ± 9.91 |
