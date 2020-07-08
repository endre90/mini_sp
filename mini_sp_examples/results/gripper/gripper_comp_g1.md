# gripper_comp_g1 commands and results

In gripper_comp_g1, there are 3 variable groups (r - robot, g - gripper, b - all balls) 

### 1 ball - all refinement order permutations:

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g1_1_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 20 1 g b r' 'cargo run --release --example gripper_comp_g1 20 1 g r b' 'cargo run --release --example gripper_comp_g1 20 1 r b g' 'cargo run --release --example gripper_comp_g1 20 1 r g b' 'cargo run --release --example gripper_comp_g1 20 1 b g r' 'cargo run --release --example gripper_comp_g1 20 1 b r g'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 1 g b r` | 74.9 ± 1.5 | 71.9 | 78.0 | 1.14 ± 0.04 |
| `cargo run --release --example gripper_comp_g1 20 1 g r b` | 66.2 ± 2.6 | 63.3 | 77.1 | 1.01 ± 0.05 |
| `cargo run --release --example gripper_comp_g1 20 1 r b g` | 79.6 ± 2.1 | 77.1 | 85.4 | 1.21 ± 0.04 |
| `cargo run --release --example gripper_comp_g1 20 1 r g b` | 65.9 ± 1.6 | 63.3 | 71.7 | 1.00 |
| `cargo run --release --example gripper_comp_g1 20 1 b g r` | 92.2 ± 1.9 | 89.5 | 96.5 | 1.40 ± 0.05 |
| `cargo run --release --example gripper_comp_g1 20 1 b r g` | 100.5 ± 5.3 | 93.8 | 110.6 | 1.52 ± 0.09 |

---

### 2 balls - all refinement order permutations:

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g1_2_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 20 2 g b r' 'cargo run --release --example gripper_comp_g1 20 2 g r b' 'cargo run --release --example gripper_comp_g1 20 2 r b g' 'cargo run --release --example gripper_comp_g1 20 2 r g b' 'cargo run --release --example gripper_comp_g1 20 2 b g r' 'cargo run --release --example gripper_comp_g1 20 2 b r g'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 2 g b r` | 92.6 ± 1.6 | 88.8 | 96.3 | 1.33 ± 0.04 |
| `cargo run --release --example gripper_comp_g1 20 2 g r b` | 71.7 ± 1.4 | 68.9 | 75.1 | 1.03 ± 0.03 |
| `cargo run --release --example gripper_comp_g1 20 2 r b g` | 4407.0 ± 53.7 | 4345.9 | 4479.3 | 63.16 ± 1.73 |
| `cargo run --release --example gripper_comp_g1 20 2 r g b` | 69.8 ± 1.7 | 67.5 | 76.8 | 1.00 |
| `cargo run --release --example gripper_comp_g1 20 2 b g r` | 135.6 ± 1.1 | 132.9 | 137.4 | 1.94 ± 0.05 |
| `cargo run --release --example gripper_comp_g1 20 2 b r g` | 152.9 ± 2.0 | 150.9 | 160.3 | 2.19 ± 0.06 |

---

### 3 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g1_3_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 20 3 g b r' 'cargo run --release --example gripper_comp_g1 20 3 g r b' 'cargo run --release --example gripper_comp_g1 20 3 r g b' 'cargo run --release --example gripper_comp_g1 20 3 b g r' 'cargo run --release --example gripper_comp_g1 20 3 b r g'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 3 g b r` | 128.2 ± 1.7 | 126.0 | 132.4 | 1.00 |
| `cargo run --release --example gripper_comp_g1 20 3 g r b` | 229.6 ± 2.4 | 226.8 | 235.4 | 1.79 ± 0.03 |
| `cargo run --release --example gripper_comp_g1 20 3 r g b` | 229.5 ± 1.7 | 226.9 | 232.9 | 1.79 ± 0.03 |
| `cargo run --release --example gripper_comp_g1 20 3 b g r` | 4146.5 ± 35.9 | 4110.1 | 4187.9 | 32.35 ± 0.51 |
| `cargo run --release --example gripper_comp_g1 20 3 b r g` | 39445.5 ± 472.3 | 38865.7 | 40146.4 | 307.71 ± 5.47 |

---

### 4 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow <br/> Removed permutation 'b g r' -> too slow <br/> Removed permutation 'b r g' -> too slow <br/>

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g1_4_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 20 4 g b r' 'cargo run --release --example gripper_comp_g1 20 4 g r b' 'cargo run --release --example gripper_comp_g1 20 4 r g b'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 4 g b r` | 238.7 ± 2.5 | 235.4 | 242.8 | 1.00 |
| `cargo run --release --example gripper_comp_g1 20 4 g r b` | 879.4 ± 9.6 | 869.1 | 894.5 | 3.68 ± 0.06 |
| `cargo run --release --example gripper_comp_g1 20 4 r g b` | 878.3 ± 7.4 | 869.8 | 886.6 | 3.68 ± 0.05 |

---

### 5 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow <br/> Removed permutation 'b g r' -> too slow <br/> Removed permutation 'b r g' -> too slow <br/>

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g1_5_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 20 5 g b r' 'cargo run --release --example gripper_comp_g1 20 5 g r b' 'cargo run --release --example gripper_comp_g1 20 5 r g b'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 5 g b r` | 948.0 ± 7.5 | 941.6 | 956.7 | 1.00 |
| `cargo run --release --example gripper_comp_g1 20 5 g r b` | 12433.0 ± 61.0 | 12349.7 | 12514.4 | 13.11 ± 0.12 |
| `cargo run --release --example gripper_comp_g1 20 5 r g b` | 13028.4 ± 265.8 | 12842.9 | 13495.8 | 13.74 ± 0.30 |

---

### 6 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow <br/> Removed permutation 'b g r' -> too slow <br/> Removed permutation 'b r g' -> too slow <br/> Removed permutation 'g r b' -> too slow <br/> Removed permutation 'r g b' -> too slow

```
hyperfine -w 3 -i -m 5 --export-markdown 'gripper_comp_g1_6_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 20 6 g b r'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 6 g b r` | 11.616 ± 0.284 | 11.185 | 11.889 | 1.00 |

---

### 7 balls - all refinement order permutations:
##### Removed permutation 'r b g' -> too slow <br/> Removed permutation 'b g r' -> too slow <br/> Removed permutation 'b r g' -> too slow <br/> Removed permutation 'g r b' -> too slow <br/> Removed permutation 'r g b' -> too slow

```
hyperfine -w 2 -i -m 3 --export-markdown 'gripper_comp_g1_7_ball_all_permutations.md' 'cargo run --release --example gripper_comp_g1 50 7 g b r'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --example gripper_comp_g1 20 7 g b r` | 212030.2 ± 2172.3 | 210494.1 | 213566.3 | 4888.15 ± 270.10 |
---

### 8 balls - haven't checked, probably timeout
---