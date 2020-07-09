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

### All instances:

```
hyperfine -w 2 -i -m 3 --export-markdown 'gripper_comp_g3_instances.md' 'cargo run --release --example gripper_comp_g3 50 2 g b r' 'cargo run --release --example gripper_comp_g3 50 4 g b r' 'cargo run --release --example gripper_comp_g3 50 6 g b r' 'cargo run --release --example gripper_comp_g3 50 8 g b r' 'cargo run --release --example gripper_comp_g3 50 10 g b r' 'cargo run --release --example gripper_comp_g3 50 12 g b r' 'cargo run --release --example gripper_comp_g3 50 14 g b r' 'cargo run --release --example gripper_comp_g3 50 16 g b r' 'cargo run --release --example gripper_comp_g3 50 18 g b r' 'cargo run --release --example gripper_comp_g3 50 20 g b r' 'cargo run --release --example gripper_comp_g3 50 22 g b r' 'cargo run --release --example gripper_comp_g3 50 24 g b r' 'cargo run --release --example gripper_comp_g3 50 26 g b r' 'cargo run --release --example gripper_comp_g3 50 28 g b r' 'cargo run --release --example gripper_comp_g3 50 30 g b r' 'cargo run --release --example gripper_comp_g3 50 32 g b r' 'cargo run --release --example gripper_comp_g3 50 34 g b r' 'cargo run --release --example gripper_comp_g3 50 36 g b r' 'cargo run --release --example gripper_comp_g3 50 38 g b r' 'cargo run --release --example gripper_comp_g3 50 40 g b r'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g3 50 2 g b r` | 88.7 ± 1.0 | 86.6 | 91.2 | 1.00 |
| `cargo run --release --example gripper_comp_g3 50 4 g b r` | 172.7 ± 1.6 | 170.4 | 175.9 | 1.95 ± 0.03 |
| `cargo run --release --example gripper_comp_g3 50 6 g b r` | 341.3 ± 1.5 | 339.8 | 344.1 | 3.85 ± 0.05 |
| `cargo run --release --example gripper_comp_g3 50 8 g b r` | 597.4 ± 4.6 | 592.7 | 604.8 | 6.74 ± 0.09 |
| `cargo run --release --example gripper_comp_g3 50 10 g b r` | 999.8 ± 3.2 | 996.4 | 1002.8 | 11.28 ± 0.13 |
| `cargo run --release --example gripper_comp_g3 50 12 g b r` | 1716.4 ± 26.5 | 1688.0 | 1740.3 | 19.36 ± 0.37 |
| `cargo run --release --example gripper_comp_g3 50 14 g b r` | 2452.7 ± 28.1 | 2421.8 | 2476.7 | 27.67 ± 0.45 |
| `cargo run --release --example gripper_comp_g3 50 16 g b r` | 3818.9 ± 12.5 | 3807.9 | 3832.6 | 43.08 ± 0.51 |
| `cargo run --release --example gripper_comp_g3 50 18 g b r` | 5484.7 ± 57.3 | 5443.7 | 5550.2 | 61.86 ± 0.95 |
| `cargo run --release --example gripper_comp_g3 50 20 g b r` | 7867.1 ± 333.7 | 7627.0 | 8248.2 | 88.74 ± 3.90 |
| `cargo run --release --example gripper_comp_g3 50 22 g b r` | 10658.6 ± 22.3 | 10637.7 | 10682.0 | 120.22 ± 1.38 |
| `cargo run --release --example gripper_comp_g3 50 24 g b r` | 14064.8 ± 83.1 | 14006.6 | 14160.0 | 158.64 ± 2.03 |
| `cargo run --release --example gripper_comp_g3 50 26 g b r` | 18409.6 ± 78.7 | 18328.2 | 18485.4 | 207.65 ± 2.51 |
| `cargo run --release --example gripper_comp_g3 50 28 g b r` | 23883.6 ± 295.7 | 23596.3 | 24186.9 | 269.39 ± 4.52 |
| `cargo run --release --example gripper_comp_g3 50 30 g b r` | 30211.9 ± 135.5 | 30065.0 | 30332.1 | 340.77 ± 4.15 |
| `cargo run --release --example gripper_comp_g3 50 32 g b r` | 38223.4 ± 2243.5 | 36869.8 | 40813.1 | 431.14 ± 25.77 |
| `cargo run --release --example gripper_comp_g3 50 34 g b r` | 47334.4 ± 27.8 | 47317.3 | 47366.5 | 533.91 ± 6.06 |
| `cargo run --release --example gripper_comp_g3 50 36 g b r` | 59086.9 ± 272.7 | 58772.1 | 59253.1 | 666.47 ± 8.15 |
| `cargo run --release --example gripper_comp_g3 50 38 g b r` | 72103.7 ± 752.6 | 71495.1 | 72945.2 | 813.29 ± 12.53 |
| `cargo run --release --example gripper_comp_g3 50 40 g b r` | 86587.0 ± 209.7 | 86373.2 | 86792.5 | 976.65 ± 11.31 |

---