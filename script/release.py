import os
import zipfile
import sys

def create_zip(sym_ver: str):
    with zipfile.ZipFile(f"archive/pt_rs_{sym_ver}.zip", 'w') as f:
        f.writestr(zipfile.ZipInfo(f"config/"), "")
        f.writestr(zipfile.ZipInfo(f"data/"), "")
        f.write("target/release/pt-rs.exe", f"pt_rs.exe")
        f.write("README.md", "README.md")

if __name__ == "__main__":
    create_zip(sys.argv[1])
