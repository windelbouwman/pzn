
import io
from ppci.lang.python import python_to_wasm

# draw_rectangle = None

#src = """
def draw():
    draw_rectangle(10, 10, 100, 100)
    draw_rectangle(110, 110, 80, 20)
    draw_rectangle(10, 110, 50, 50)
#"""

# f = io.StringIO(src)
imports = ['(import "env" "draw_rectangle" (func $draw_rectangle (param f64 f64 f64 f64)))']
mod = python_to_wasm(draw, imports=imports)
print(mod)

with open('shader.wasm', 'wb') as f:
    mod.to_file(f)