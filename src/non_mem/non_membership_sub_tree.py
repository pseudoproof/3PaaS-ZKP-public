## code from zkmb with modifications
from poseidon_hash import poseidon_hash
import os
import time
import ast

blocklist_file_name = "sub_tree/input_list.txt"

# read blacklist from file
def parseFile(filename):
    allList = []
    max_len = 0
    max_str = ""
    with open(filename, 'rb') as readFile:
        for line in readFile:
            line = str(line)
            line = line.split("'")[1].split("\\n")[0]
            allList.append(int(line))
    return allList

def parseFile_sorted(filename):
    blocklist = []
    max_len = 0
    max_str = ""
    with open(filename, 'rb') as readFile:
        for line in readFile:
            line = str(line)
            line = line.split("'")[1].split("\\n")[0]
            blocklist.append(int(line))
    sorted_list = sorted(blocklist)
    return sorted_list

def blocklist_to_hash_leaves(set_input):
    output_hash = []
    set_input
    for ele in set_input:
        output_hash.append(poseidon_hash([ele]))
    return output_hash

# compute merkle tree height
def merkle_tree_height(input_length):
    input_length = int(input_length)
    bit_length = input_length.bit_length()
    if bit_length == 0:
        return 0
    elif (1<<(bit_length-1)) == (input_length):
        return bit_length-1
    else:
        return bit_length

# compute merkle tree root, merkle tree, height 
def compute_merkle_tee(set_input):
    height = merkle_tree_height(len(set_input))
    # pad leaves to the power of 2
    complete_input = set_input + [0 for i in range(0, (1<<height)-len(set_input))]
    
    interval = height
    start_index = 0

    for i in range(0, height):
        for j in range(start_index, start_index + (1<<interval), 2):
            complete_input.append(poseidon_hash([complete_input[j],complete_input[j+1]]))
        start_index += 1<<interval
        interval = interval-1

    return (complete_input[-1], complete_input, height)


# get merkle tree path
def get_merkle_tree_path(merkle_tree, height, dirSelection):
    dirSelection = int(dirSelection)
    direction = []
    auth_path = []
    start_index = 0
    interval = height
    for i in range(0, height):
        if (dirSelection%2) == 1:
            auth_path.append(merkle_tree[start_index + dirSelection - 1])
            direction.append(1)
        else:
            auth_path.append(merkle_tree[start_index + dirSelection + 1])
            direction.append(0)

        start_index += (1<<interval)
        interval = interval-1
        dirSelection = dirSelection>>1

    return direction, auth_path

sorted_list = "sub_tree/sorted_list.txt"
sorted_list_w_hash = "sub_tree/sorted_hash_list.txt"

def sorted_hash_list(blocklist_file_name):
    parsed = parseFile_sorted(blocklist_file_name)

    f = open(sorted_list,"w")
    for line in parsed:
        f.write(str(line)+'\n')
    f.close()

    hashed_leaves = blocklist_to_hash_leaves(parsed)

    f = open(sorted_list_w_hash,"w")
    for line in hashed_leaves:
        f.write(str(line)+'\n')
    f.close()

def gen_merkle_tree(file_name):
    merkle_input_hashed = parseFile(file_name)
    height = merkle_tree_height(len(merkle_input_hashed))
    tree_struct = compute_merkle_tee(merkle_input_hashed)
    f = open("sub_tree/merkle_tree.txt","w")
    f.write(str(tree_struct))
    f.close()

def get_dir_and_auth_path(leaf_idx):
    with open("sub_tree/merkle_tree.txt", 'rb') as readFile:
        tree_struct = eval(readFile.read())
    height = tree_struct[2]
    direction, auth_path = get_merkle_tree_path(tree_struct[1], height, leaf_idx)
    return height, direction, auth_path, tree_struct[0]

# verify merkle path
def verify_path(leaf, auth_path, direction, height, root):
    currentDigest = poseidon_hash(leaf)
    for i in range(0, height):
        if direction[i] == 1:
            inputToNextHash = [auth_path[i], currentDigest]
        else:
            inputToNextHash = [currentDigest, auth_path[i]]
        currentDigest = poseidon_hash(inputToNextHash)
    return (currentDigest == root)

# run this to init the blocklist
# create sorted hash leaves
sorted_hash_list(blocklist_file_name)

# create merkle tree from sorted hash leaves
gen_merkle_tree(sorted_list_w_hash)

# non-membership proof
if __name__ == "__main__":
    with open("../zkp/prover_inputs/input_poseidon.txt", "r") as file:
        ID_A = ast.literal_eval(file.readline().strip())

    leaf = 0
    for i in range(0,8):
        leaf = 2**(i*8)*(ID_A[i])  + leaf

    inputs = parseFile(sorted_list)
    for i in range(0,len(inputs)):
        if (leaf < inputs[i]):
            left_leaf = inputs[i-1]
            right_leaf = inputs[i]
            break
        elif (leaf == inputs[i]):
            print("invalid input in the blocklist!!!")
            exit()
    authPath_left = get_dir_and_auth_path(i-1)
    authPath_right = get_dir_and_auth_path(i)

    values=[left_leaf,authPath_left[1],authPath_left[2],right_leaf,authPath_right[1],authPath_right[2],authPath_left[3]]

    print("Creating token_revocation.txt ....")
    f = open("../zkp/prover_inputs/token_revocation.txt","w")
    for line in values:
        f.write(str(line)+'\n')
    f.write(str(ID_A)+'\n')
    f.close()

    # print("left_leaf",left_leaf)
    # print("direction_left",authPath_left[1])
    # print("auth_path_left",authPath_left[2])
    # print("right_leaf", right_leaf)
    # print("direction_right",authPath_right[1])
    # print("auth_path_right",authPath_right[2])
    # print("root", authPath_left[3])


# membership proof
# if __name__ == "__main__":
# 	leaf = 7846963901164791998
# 	leaf_idx = parseFile(sorted_list).index(leaf)
# 	height, direction, auth_path, root = get_dir_and_auth_path(leaf_idx)

# 	## verify_path for leaf
# 	print(verify_path([leaf], auth_path, direction, height, root))

# 	# height
# 	print(height)

# 	# direction
# 	print(direction)

# 	# auth_path
# 	print(auth_path)

# 	# root
# 	print(root)