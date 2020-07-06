| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 3 g b r` | 120.4 ± 1.3 | 118.1 | 122.9 | 1.00 |
| `cargo run --release --example gripper_comp 20 3 g r b` | 220.2 ± 2.7 | 216.6 | 225.1 | 1.83 ± 0.03 |
| `cargo run --release --example gripper_comp 20 3 r g b` | 218.9 ± 1.7 | 216.8 | 223.1 | 1.82 ± 0.02 |

### Comment: Other combinations removed (no plan found)