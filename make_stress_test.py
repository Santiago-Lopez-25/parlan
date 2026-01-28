import subprocess
import time
import random
import sys

# CONFIGURACIÓN
PATH_PROYECTO_R = "C:/Users/Graciela/parlan_rust/target/release/parlan_rust.exe" # <--- Tu ruta absoluta aquí
PATH_PROYECTO_D = "C:/Users/Graciela/parlan_rust/target/debug/parlan_rust.exe"
NOMBRE_BINARIO = "parlan_rust" # El nombre que aparece en tu Cargo.toml
ARCHIVO_TEST = "main.par"
ITERACIONES = 1
LINEAS_INICIALES = 1000000

# Construimos la ruta al binario de release
#RUTE_BINARIO = os.path.join(PATH_PROYECTO, "target", "release", NOMBRE_BINARIO)

def preparar_compilador_release():
    print("Compilando proyecto en modo release...")
    subprocess.run(["cargo", "build", "--release"], capture_output=True , cwd="C:/Users/Graciela/parlan_rust", check=True)

def preparar_compilador_debug():
    print("Compilando proyecto en modo debug...")
    subprocess.run(["cargo", "build", "--quiet"] , cwd="C:/Users/Graciela/parlan_rust", check=True)

def generar_codigo(num_bloques):
    with open(ARCHIVO_TEST, 'w') as f:
        for i in range(num_bloques):
            # 1. Definir una función
            f.write(f"func operacion_{i}(n: int): int {{\n")
            f.write(f"    var temp: int = n * 2\n")
            f.write(f"    if temp > 100 {{\n")
            f.write(f"        return temp + {i}\n")
            f.write(f"    }}\n")
            f.write(f"    var res_{i}: int = operacion_{i}({random.randint(1, 50)})\n")
            f.write(f"    if res_{i} < 1000 {{\n")
            f.write(f"        res_{i} = res_{i} + 1\n")
            f.write(f"    }}\n\n")
            f.write(f"    return temp\n")
            f.write(f"}}\n\n")
            
            # 2. Llamarla en un condicional
            

def ejecutar_benchmark_release(num_lineas):
    generar_codigo(num_lineas)
    
    # Medimos solo la ejecución del binario, no de Cargo
    proceso = subprocess.run(
        [PATH_PROYECTO_R, "--time"],
        text=True
    )

def ejecutar_benchmark_debug(num_lineas):
    generar_codigo(num_lineas)
    
    # Medimos solo la ejecución del binario, no de Cargo
    proceso = subprocess.run(
        [PATH_PROYECTO_D, "--time"],
        text=True
    )


# --- FLUJO PRINCIPAL ---
def correr_compilador_debug():
    preparar_compilador_debug()
    lineas = LINEAS_INICIALES
    for _ in range(ITERACIONES):
        ejecutar_benchmark_debug(lineas)
    
        lineas *= 2

#print()

def correr_compilador_release():
    preparar_compilador_release()
    lineas = LINEAS_INICIALES
    for _ in range(ITERACIONES):
        ejecutar_benchmark_release(lineas)

        lineas *= 2

n = int(sys.argv[1] if len(sys.argv) > 1 else -1)
if n == -1: 
    print("expected 1 argument: number of blocks (1 block = 13 lines)")
else:
    print(f"generating {n * 13} lines")
    generar_codigo(n)