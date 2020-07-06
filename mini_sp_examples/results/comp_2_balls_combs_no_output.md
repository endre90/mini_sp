| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 2 g b r` | 79.4 ± 0.8 | 77.9 | 81.4 | 1.36 ± 0.02 |
| `cargo run --release --example gripper_comp 20 2 g r b` | 59.0 ± 2.0 | 56.8 | 66.7 | 1.01 ± 0.04 |
| `cargo run --release --example gripper_comp 20 2 r g b` | 58.3 ± 0.9 | 56.6 | 60.5 | 1.00 |
| `cargo run --release --example gripper_comp 20 2 r b g` | 4445.1 ± 119.5 | 4239.7 | 4575.9 | 76.20 ± 2.35 |
| `cargo run --release --example gripper_comp 20 2 b g r` | 127.0 ± 3.0 | 122.4 | 130.3 | 2.18 ± 0.06 |
| `cargo run --release --example gripper_comp 20 2 b r g` | 139.5 ± 1.7 | 136.8 | 143.0 | 2.39 ± 0.05 |
