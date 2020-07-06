| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 2 g b r` | 82.8 ± 2.0 | 78.6 | 88.2 | 1.39 ± 0.06 |
| `cargo run --release --example gripper_comp 20 2 g r b` | 59.9 ± 0.9 | 58.6 | 63.2 | 1.01 ± 0.04 |
| `cargo run --release --example gripper_comp 20 2 r g b` | 59.6 ± 2.0 | 57.4 | 68.0 | 1.00 |
| `cargo run --release --example gripper_comp 20 2 r b g` | 4564.5 ± 163.7 | 4337.8 | 4792.8 | 76.63 ± 3.77 |
| `cargo run --release --example gripper_comp 20 2 b g r` | 135.3 ± 13.6 | 120.6 | 167.8 | 2.27 ± 0.24 |
| `cargo run --release --example gripper_comp 20 2 b r g` | 139.8 ± 2.4 | 135.8 | 145.3 | 2.35 ± 0.09 |
