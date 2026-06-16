import random
import os

os.makedirs('sub_tree', exist_ok=True)
# Generate a random 8-byte integer
f = open("sub_tree/input_list.txt","w")
for i in range(0,2**8):
    num = random.randint(0, 2**64 - 1)
    f.write(str(num)+'\n')
f.close()

