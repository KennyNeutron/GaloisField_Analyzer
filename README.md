# GaloisField_Analyzer

Rust implementation for CoE 164 Week 07EA — formatting polynomials in GF(2^8) using an irreducible polynomial.

---

## How to Run

### 1. Build the Project

Make sure you're in the project folder, then run:

```bash
cargo build
```

---

### 2. Run the Program with Input File

#### PowerShell (Windows):

```powershell
Get-Content in_pub.txt | cargo run | Out-File out_pub.txt
```

#### Bash (Linux/macOS):

```bash
cargo run < in_pub.txt > out_pub.txt
```

---

### 3. Verify the Output

#### PowerShell (Windows):

```powershell
Compare-Object (Get-Content out_pub.txt) (Get-Content out_pub_ans.txt)
```

#### Bash (Linux/macOS):

```bash
diff out_pub.txt out_pub_ans.txt
```

If there is no output from the comparison, the results match and are correct.

---

## Notes

- `in_pub.txt` contains the input test case(s)
- `out_pub.txt` is your program’s actual output
- `out_pub_ans.txt` contains the expected output for comparison
