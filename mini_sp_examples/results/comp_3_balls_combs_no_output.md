| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 3 g b r` | 114.6 ± 1.6 | 112.0 | 118.9 | 1.00 |
| `cargo run --release --example gripper_comp 20 3 g r b` | 216.3 ± 2.0 | 213.4 | 219.7 | 1.89 ± 0.03 |
| `cargo run --release --example gripper_comp 20 3 r g b` | 215.6 ± 2.2 | 211.8 | 218.2 | 1.88 ± 0.03 |

### Comment: Other combinations removed (no plan found)