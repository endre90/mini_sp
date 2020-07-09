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

### All instances:

```
hyperfine -w 2 -i -m 3 --export-markdown 'gripper_comp_g2_instances.md' 'cargo run --release --example gripper_comp_g2 50 2 g b r' 'cargo run --release --example gripper_comp_g2 50 4 g b r' 'cargo run --release --example gripper_comp_g2 50 6 g b r' 'cargo run --release --example gripper_comp_g2 50 8 g b r' 'cargo run --release --example gripper_comp_g2 50 10 g b r' 'cargo run --release --example gripper_comp_g2 50 12 g b r' 'cargo run --release --example gripper_comp_g2 50 14 g b r' 'cargo run --release --example gripper_comp_g2 50 16 g b r' 'cargo run --release --example gripper_comp_g2 50 18 g b r' 'cargo run --release --example gripper_comp_g2 50 20 g b r' 'cargo run --release --example gripper_comp_g2 50 22 g b r' 'cargo run --release --example gripper_comp_g2 50 24 g b r' 'cargo run --release --example gripper_comp_g2 50 26 g b r' 'cargo run --release --example gripper_comp_g2 50 28 g b r' 'cargo run --release --example gripper_comp_g2 50 30 g b r' 'cargo run --release --example gripper_comp_g2 50 32 g b r' 'cargo run --release --example gripper_comp_g2 50 34 g b r' 'cargo run --release --example gripper_comp_g2 50 36 g b r' 'cargo run --release --example gripper_comp_g2 50 38 g b r' 'cargo run --release --example gripper_comp_g2 50 40 g b r'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g2 50 2 g b r` | 108.9 ± 1.1 | 107.5 | 112.5 | 1.00 |
| `cargo run --release --example gripper_comp_g2 50 4 g b r` | 239.2 ± 0.9 | 238.1 | 241.3 | 2.20 ± 0.02 |
| `cargo run --release --example gripper_comp_g2 50 6 g b r` | 500.3 ± 3.8 | 497.1 | 507.4 | 4.59 ± 0.06 |
| `cargo run --release --example gripper_comp_g2 50 8 g b r` | 935.1 ± 6.4 | 928.0 | 940.3 | 8.58 ± 0.10 |
| `cargo run --release --example gripper_comp_g2 50 10 g b r` | 1650.3 ± 33.5 | 1618.0 | 1684.9 | 15.15 ± 0.34 |
| `cargo run --release --example gripper_comp_g2 50 12 g b r` | 2741.0 ± 24.0 | 2713.3 | 2755.3 | 25.16 ± 0.33 |
| `cargo run --release --example gripper_comp_g2 50 14 g b r` | 4161.3 ± 68.7 | 4120.2 | 4240.6 | 38.20 ± 0.73 |
| `cargo run --release --example gripper_comp_g2 50 16 g b r` | 6356.6 ± 142.8 | 6191.9 | 6447.6 | 58.35 ± 1.43 |
| `cargo run --release --example gripper_comp_g2 50 18 g b r` | 9494.7 ± 40.9 | 9458.3 | 9539.0 | 87.16 ± 0.92 |
| `cargo run --release --example gripper_comp_g2 50 20 g b r` | 13428.4 ± 100.4 | 13339.6 | 13537.4 | 123.27 ± 1.51 |
| `cargo run --release --example gripper_comp_g2 50 22 g b r` | 18238.4 ± 72.8 | 18171.5 | 18316.0 | 167.43 ± 1.76 |
| `cargo run --release --example gripper_comp_g2 50 24 g b r` | 24687.9 ± 111.6 | 24560.8 | 24769.9 | 226.63 ± 2.42 |
| `cargo run --release --example gripper_comp_g2 50 26 g b r` | 32047.8 ± 1100.8 | 30805.1 | 32900.5 | 294.20 ± 10.50 |
| `cargo run --release --example gripper_comp_g2 50 28 g b r` | 42523.8 ± 59.0 | 42472.0 | 42588.1 | 390.36 ± 3.82 |
| `cargo run --release --example gripper_comp_g2 50 30 g b r` | 54559.4 ± 607.7 | 54157.3 | 55258.4 | 500.85 ± 7.39 |
| `cargo run --release --example gripper_comp_g2 50 32 g b r` | 68148.1 ± 181.8 | 67948.2 | 68303.5 | 625.59 ± 6.29 |
| `cargo run --release --example gripper_comp_g2 50 34 g b r` | 84815.0 ± 413.9 | 84471.2 | 85274.3 | 778.59 ± 8.45 |
| `cargo run --release --example gripper_comp_g2 50 36 g b r` | 105384.3 ± 633.1 | 104895.7 | 106099.6 | 967.42 ± 11.03 |
| `cargo run --release --example gripper_comp_g2 50 38 g b r` | 128208.0 ± 669.2 | 127704.5 | 128967.4 | 1176.94 ± 12.96 |
| `cargo run --release --example gripper_comp_g2 50 40 g b r` | 152815.3 ± 3783.6 | 148447.5 | 155086.3 | 1402.83 ± 37.30 |

---