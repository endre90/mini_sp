| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 4 g b r` | 228.1 ± 3.5 | 223.4 | 234.1 | 1.00 |
| `cargo run --release --example gripper_comp 20 4 g r b` | 891.0 ± 36.6 | 853.7 | 960.2 | 3.91 ± 0.17 |
| `cargo run --release --example gripper_comp 20 4 r g b` | 879.6 ± 7.5 | 871.7 | 894.5 | 3.86 ± 0.07 |

### Comment: Other combinations removed (no plan found)