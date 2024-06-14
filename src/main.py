import my_rust_lib
import random

ans = [[random.randint(1, 19) for _ in range(3)] for _ in range(10000)]

ans = my_rust_lib.score_and_update_values(ans)
print("Hello from Python")

print(ans)