# Open the source file in read mode and the destination file in append mode
with open('token_revocation.txt', 'r') as source_file, open('input_poseidon.txt', 'a') as destination_file:
    # Loop through each line in the source file
    for line in source_file:
        # Append the line to the destination file
        destination_file.write(line)

# with open('token_revocation.txt', 'r') as source_file, open('input_sha.txt', 'a') as destination_file:
#     # Loop through each line in the source file
#     for line in source_file:
#         # Append the line to the destination file
#         destination_file.write(line)