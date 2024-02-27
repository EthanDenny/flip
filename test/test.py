import subprocess
from subprocess import DEVNULL, STDOUT

def run_test(file, target):
    subprocess.check_call(["cargo", "run", file], stdout=DEVNULL, stderr=STDOUT)
    subprocess.check_call(["gcc", "build/out.c", "-I", "src/", "-o", "build/out"], stdout=DEVNULL, stderr=STDOUT)
    stdout = subprocess.check_output("./build/out")
    return str(stdout)[2:-3] == target

if __name__ == "__main__":
    tests = [
        ("test/e1.flip", "233168"),
        ("test/e2.flip", "4613732"),
        ("test/primes.flip", "111587")
    ]

    max_length = max(map(lambda x: len(x[0]), tests))

    for (name, target) in tests:
        result = "PASS" if run_test(name, target) else "FAIL"
        print(f"{(name + ':').ljust(max_length+1):<1} {result}")
