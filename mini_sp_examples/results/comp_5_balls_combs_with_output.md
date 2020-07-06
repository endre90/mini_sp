| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp 20 5 g b r` | 939.6 ± 19.5 | 911.8 | 980.6 | 1.00 |
| `cargo run --release --example gripper_comp 20 5 g r b` | 12929.6 ± 331.1 | 12452.8 | 13505.1 | 13.76 ± 0.45 |
| `cargo run --release --example gripper_comp 20 5 r g b` | 12806.0 ± 298.9 | 12083.2 | 13098.7 | 13.63 ± 0.43 |

### Comment: Other combinations removed (no plan found)