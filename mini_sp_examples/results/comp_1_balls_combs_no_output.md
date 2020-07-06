| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 1 g b r` | 62.3 ± 1.1 | 60.7 | 66.6 | 1.18 ± 0.04 |
| `cargo run --release --example gripper_comp 20 1 g r b` | 52.9 ± 1.2 | 51.2 | 56.0 | 1.00 ± 0.03 |
| `cargo run --release --example gripper_comp 20 1 r g b` | 52.7 ± 1.2 | 51.1 | 57.8 | 1.00 |
| `cargo run --release --example gripper_comp 20 1 r b g` | 67.3 ± 0.8 | 65.8 | 69.3 | 1.28 ± 0.03 |
| `cargo run --release --example gripper_comp 20 1 b g r` | 79.4 ± 1.1 | 77.9 | 82.8 | 1.51 ± 0.04 |
| `cargo run --release --example gripper_comp 20 1 b r g` | 83.6 ± 1.0 | 82.0 | 86.2 | 1.59 ± 0.04 |
