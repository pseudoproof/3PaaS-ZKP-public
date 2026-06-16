import random
import os

os.makedirs('root', exist_ok=True)
# Generate a random 8-byte integer
f = open("root/input_list_roots.txt","w")
for i in range(0,(2**22)-1):
    num = random.randint(0, 2**255 - 1)
    f.write(str(num)+'\n')
f.close()

