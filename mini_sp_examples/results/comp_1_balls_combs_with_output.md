| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 1 g b r` | 64.4 ± 1.5 | 62.4 | 71.4 | 1.20 ± 0.04 |
| `cargo run --release --example gripper_comp 20 1 g r b` | 53.6 ± 1.0 | 52.0 | 57.4 | 1.00 |
| `cargo run --release --example gripper_comp 20 1 r g b` | 58.4 ± 5.4 | 52.1 | 81.3 | 1.09 ± 0.10 |
| `cargo run --release --example gripper_comp 20 1 r b g` | 70.6 ± 2.2 | 67.5 | 74.7 | 1.32 ± 0.05 |
| `cargo run --release --example gripper_comp 20 1 b g r` | 84.6 ± 4.3 | 80.6 | 98.8 | 1.58 ± 0.08 |
| `cargo run --release --example gripper_comp 20 1 b r g` | 93.4 ± 3.8 | 88.3 | 101.4 | 1.74 ± 0.08 |
