import subprocess
import random
import sys

PROYECT_PATH = "parlan_realease_test.exe"
TEST_FILE = "main.par"


def generar_codigo(num_bloques):
    with open(TEST_FILE, 'w') as f:
        buff = ""
        for i in range(num_bloques):
            f.write(f"""func operation_{i}(n: int): int {{
    var temp: int = n * 2
    if temp > 100 {{
        return temp + {i}
    }}
    var res_{i}: int = operation_{i}({random.randint(1,50)})
    if res_{i} < 1000 {{
        res_{i} = res_{i} + 1
    }}

    return temp
}}

""")
            
def run_compiler():
    subprocess.run(
        [PROYECT_PATH,"main.par","--time"],
        text=True
    )

n = int(sys.argv[1] if len(sys.argv) > 1 else -1)
if n == -1: 
    print("expected 1 argument: number of lines")
else:
    if n < 13:
        print("cannot make less than 13 lines. rounding to 13 lines")
        n = 13
    print(f"generating {(n // 13) * 13} lines")
    generar_codigo(n // 13)
    run_compiler()
