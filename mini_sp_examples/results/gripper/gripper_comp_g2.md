# gripper_comp_g2 commands and results

In gripper_comp_g2, there are 2 + n (n -> nr. of balls) variable groups (r - robot, g - gripper, n - balls) 

### 5 balls - all refinement order permutations:

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g2_5_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g2 50 5 g b r' 'cargo run --release --example gripper_comp_g2 50 5 g r b' 'cargo run --release --example gripper_comp_g2 50 5 r b g' 'cargo run --release --example gripper_comp_g2 50 5 r g b' 'cargo run --release --example gripper_comp_g2 50 5 b g r' 'cargo run --release --example gripper_comp_g2 50 5 b r g'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g2 50 5 g b r` | 353.8 ± 4.4 | 349.4 | 360.4 | 1.00 |
| `cargo run --release --example gripper_comp_g2 50 5 g r b` | 406.7 ± 3.0 | 402.8 | 412.2 | 1.15 ± 0.02 |
| `cargo run --release --example gripper_comp_g2 50 5 r b g` | 523.8 ± 3.6 | 519.6 | 528.2 | 1.48 ± 0.02 |
| `cargo run --release --example gripper_comp_g2 50 5 r g b` | 413.2 ± 4.7 | 407.9 | 421.0 | 1.17 ± 0.02 |
| `cargo run --release --example gripper_comp_g2 50 5 b g r` | 426.3 ± 3.3 | 422.2 | 431.9 | 1.20 ± 0.02 |
| `cargo run --release --example gripper_comp_g2 50 5 b r g` | 498.1 ± 5.2 | 491.5 | 503.2 | 1.41 ± 0.02 |

---

### 10 balls - all refinement order permutations:

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g2_10_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g2 50 10 g b r' 'cargo run --release --example gripper_comp_g2 50 10 g r b' 'cargo run --release --example gripper_comp_g2 50 10 r b g' 'cargo run --release --example gripper_comp_g2 50 10 r g b' 'cargo run --release --example gripper_comp_g2 50 10 b g r' 'cargo run --release --example gripper_comp_g2 50 10 b r g'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g2 50 10 g b r` | 1.746 ± 0.019 | 1.717 | 1.764 | 1.00 |
| `cargo run --release --example gripper_comp_g2 50 10 g r b` | 2.618 ± 0.036 | 2.588 | 2.678 | 1.50 ± 0.03 |
| `cargo run --release --example gripper_comp_g2 50 10 r b g` | 2.823 ± 0.075 | 2.722 | 2.917 | 1.62 ± 0.05 |
| `cargo run --release --example gripper_comp_g2 50 10 r g b` | 2.642 ± 0.009 | 2.628 | 2.654 | 1.51 ± 0.02 |
| `cargo run --release --example gripper_comp_g2 50 10 b g r` | 2.047 ± 0.046 | 2.019 | 2.128 | 1.17 ± 0.03 |
| `cargo run --release --example gripper_comp_g2 50 10 b r g` | 2.290 ± 0.036 | 2.260 | 2.337 | 1.31 ± 0.03 |

---

### 20 balls - all refinement order permutations:

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g2_20_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g2 50 20 g b r' 'cargo run --release --example gripper_comp_g2 50 20 g r b' 'cargo run --release --example gripper_comp_g2 50 20 r b g' 'cargo run --release --example gripper_comp_g2 50 20 r g b' 'cargo run --release --example gripper_comp_g2 50 20 b g r' 'cargo run --release --example gripper_comp_g2 50 20 b r g'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g2 50 20 g b r` | 13.309 ± 0.249 | 12.973 | 13.579 | 1.00 |
| `cargo run --release --example gripper_comp_g2 50 20 g r b` | 23.681 ± 1.707 | 22.008 | 26.556 | 1.78 ± 0.13 |
| `cargo run --release --example gripper_comp_g2 50 20 r b g` | 24.100 ± 0.483 | 23.574 | 24.666 | 1.81 ± 0.05 |
| `cargo run --release --example gripper_comp_g2 50 20 r g b` | 22.866 ± 0.647 | 22.012 | 23.781 | 1.72 ± 0.06 |
| `cargo run --release --example gripper_comp_g2 50 20 b g r` | 14.730 ± 0.611 | 13.961 | 15.586 | 1.11 ± 0.05 |
| `cargo run --release --example gripper_comp_g2 50 20 b r g` | 16.081 ± 0.643 | 15.068 | 16.664 | 1.21 ± 0.05 |

---

### 30 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow <br/> Removed permutation 'g r b' -> too slow <br/> Removed permutation 'r g b' -> too slow <br/>

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g2_30_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g2 50 30 g b r' 'cargo run --release --example gripper_comp_g2 50 30 b g r' 'cargo run --release --example gripper_comp_g2 50 30 b r g'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g2 50 30 g b r` | 60.588 ± 5.985 | 54.081 | 67.631 | 1.00 |
| `cargo run --release --example gripper_comp_g2 50 30 b g r` | 65.195 ± 1.343 | 64.048 | 67.345 | 1.08 ± 0.11 |
| `cargo run --release --example gripper_comp_g2 50 30 b r g` | 68.107 ± 3.798 | 63.721 | 72.625 | 1.12 ± 0.13 |

---

### 40 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow <br/> Removed permutation 'g r b' -> too slow <br/> Removed permutation 'r g b' -> too slow <br/>

```
hyperfine -w 2 -i -m 3 --export-markdown 'gripper_comp_g2_40_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g2 50 40 g b r' 'cargo run --release --example gripper_comp_g2 50 40 b g r' 'cargo run --release --example gripper_comp_g2 50 40 b r g'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g2 50 40 g b r` | 154.836 ± 4.085 | 151.566 | 159.416 | 1.00 |
| `cargo run --release --example gripper_comp_g2 50 40 b g r` | 156.068 ± 0.500 | 155.497 | 156.429 | 1.01 ± 0.03 |
| `cargo run --release --example gripper_comp_g2 50 40 b r g` | 164.869 ± 4.374 | 160.029 | 168.538 | 1.06 ± 0.04 |
 ---