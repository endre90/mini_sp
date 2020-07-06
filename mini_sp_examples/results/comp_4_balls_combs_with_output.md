| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 4 g b r` | 227.7 ± 1.1 | 225.8 | 230.1 | 1.00 |
| `cargo run --release --example gripper_comp 20 4 g r b` | 850.4 ± 4.5 | 844.6 | 857.8 | 3.74 ± 0.03 |
| `cargo run --release --example gripper_comp 20 4 r g b` | 855.0 ± 6.0 | 847.0 | 863.4 | 3.76 ± 0.03 |

### Comment: Other combinations removed (no plan found)