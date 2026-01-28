import random

def generar_codigo(nombre, num_bloques):
    with open(nombre, 'w') as f:
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

if __name__ == "__main__":
    generar_codigo("main.par", 1)
    print("Archivo 'stress_test.txt' generado con éxito.")