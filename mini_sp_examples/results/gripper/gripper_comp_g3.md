# gripper_comp_g3 commands and results

In gripper_comp_g3, there are (2 + (n div 2) + (n mod 2)) (n -> nr. of balls) variable groups (r - robot, g - gripper, b - balls)

### 1 ball - all refinement order permutations:

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g3_1_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 1 g b r' 'cargo run --release --example gripper_comp_g3 50 1 g r b' 'cargo run --release --example gripper_comp_g3 50 1 r b g' 'cargo run --release --example gripper_comp_g3 50 1 r g b' 'cargo run --release --example gripper_comp_g3 50 1 b g r' 'cargo run --release --example gripper_comp_g3 50 1 b r g'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 1 g b r` | 74.4 ± 1.4 | 72.4 | 78.3 | 1.16 ± 0.03 |
| `cargo run --release --example gripper_comp_g3 50 1 g r b` | 66.3 ± 4.0 | 62.8 | 88.4 | 1.03 ± 0.07 |
| `cargo run --release --example gripper_comp_g3 50 1 r b g` | 78.8 ± 1.9 | 76.3 | 86.0 | 1.23 ± 0.04 |
| `cargo run --release --example gripper_comp_g3 50 1 r g b` | 64.2 ± 1.4 | 61.6 | 67.7 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 1 b g r` | 92.0 ± 1.7 | 89.5 | 96.3 | 1.43 ± 0.04 |
| `cargo run --release --example gripper_comp_g3 50 1 b r g` | 97.3 ± 5.8 | 94.2 | 126.3 | 1.52 ± 0.10 |

---

### 2 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> no plan found

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g3_2_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 2 g b r' 'cargo run --release --example gripper_comp_g3 50 2 g r b' 'cargo run --release --example gripper_comp_g3 50 2 r g b' 'cargo run --release --example gripper_comp_g3 50 2 b g r' 'cargo run --release --example gripper_comp_g3 50 2 b r g'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 2 g b r` | 115.6 ± 2.1 | 113.3 | 122.5 | 1.32 ± 0.03 |
| `cargo run --release --example gripper_comp_g3 50 2 g r b` | 88.4 ± 1.9 | 86.3 | 95.2 | 1.01 ± 0.02 |
| `cargo run --release --example gripper_comp_g3 50 2 r g b` | 87.3 ± 1.0 | 86.0 | 91.0 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 2 b g r` | 174.4 ± 7.2 | 169.0 | 198.3 | 2.00 ± 0.09 |
| `cargo run --release --example gripper_comp_g3 50 2 b r g` | 189.7 ± 3.1 | 181.1 | 193.0 | 2.17 ± 0.04 |

---

### 5 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> no plan found <br/> Removed permutation 'b g r' -> no plan found <br/> Removed permutation 'b r g' -> no plan found

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g3_5_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 5 g b r' 'cargo run --release --example gripper_comp_g3 50 5 g r b' 'cargo run --release --example gripper_comp_g3 50 5 r g b'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 5 g b r` | 277.5 ± 0.8 | 276.3 | 278.6 | 1.11 ± 0.01 |
| `cargo run --release --example gripper_comp_g3 50 5 g r b` | 250.9 ± 3.1 | 246.8 | 258.9 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 5 r g b` | 252.7 ± 2.5 | 249.9 | 258.1 | 1.01 ± 0.02 |

---

### 10 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> no plan found <br/> Removed permutation 'b g r' -> no plan found <br/> Removed permutation 'b r g' -> no plan found

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g3_10_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 10 g b r' 'cargo run --release --example gripper_comp_g3 50 10 g r b' 'cargo run --release --example gripper_comp_g3 50 10 r g b'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 10 g b r` | 1.141 ± 0.060 | 1.058 | 1.217 | 1.03 ± 0.07 |
| `cargo run --release --example gripper_comp_g3 50 10 g r b` | 1.103 ± 0.040 | 1.074 | 1.174 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 10 r g b` | 1.109 ± 0.006 | 1.103 | 1.118 | 1.01 ± 0.04 |

---

### 20 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> no plan found <br/> Removed permutation 'b g r' -> no plan found <br/> Removed permutation 'b r g' -> no plan found

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g3_20_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 20 g b r' 'cargo run --release --example gripper_comp_g3 50 20 g r b' 'cargo run --release --example gripper_comp_g3 50 20 r g b'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 20 g b r` | 7.931 ± 0.219 | 7.687 | 8.190 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 20 g r b` | 9.061 ± 0.131 | 8.838 | 9.176 | 1.14 ± 0.04 |
| `cargo run --release --example gripper_comp_g3 50 20 r g b` | 11.045 ± 1.381 | 9.324 | 12.856 | 1.39 ± 0.18 |

---

### 30 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> no plan found <br/> Removed permutation 'b g r' -> no plan found <br/> Removed permutation 'b r g' -> no plan found

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g3_30_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 30 g b r' 'cargo run --release --example gripper_comp_g3 50 30 g r b' 'cargo run --release --example gripper_comp_g3 50 30 r g b'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 30 g b r` | 30.787 ± 0.954 | 30.022 | 32.362 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 30 g r b` | 38.381 ± 0.543 | 37.800 | 39.237 | 1.25 ± 0.04 |
| `cargo run --release --example gripper_comp_g3 50 30 r g b` | 37.215 ± 0.103 | 37.068 | 37.330 | 1.21 ± 0.04 |

---

### 40 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> no plan found <br/> Removed permutation 'b g r' -> no plan found <br/> Removed permutation 'b r g' -> no plan found

```
hyperfine -w 2 -i -m 3 --export-markdown 'gripper_comp_g3_40_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g3 50 40 g b r' 'cargo run --release --example gripper_comp_g3 50 40 g r b' 'cargo run --release --example gripper_comp_g3 50 40 r g b'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 40 g b r` | 86.441 ± 0.294 | 86.251 | 86.780 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 40 g r b` | 106.536 ± 0.594 | 106.010 | 107.180 | 1.23 ± 0.01 |
| `cargo run --release --example gripper_comp_g3 50 40 r g b` | 106.657 ± 1.797 | 104.585 | 107.792 | 1.23 ± 0.02 |

---