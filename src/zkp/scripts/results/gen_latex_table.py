#code from ChatGPT with modifications
def array_to_latex_multicol_table(array, caption="My Table", label="tab:my_table"):
    if not array or len(array) < 2:
        return "%% Error: Need at least two rows for multicolumn header"
    
    # Calculate total columns from the first row after multicolumn adjustments
    num_cols = sum(span for _, span in array[0][1:]) + 1  # 1 for the first regular column
    column_format = "|c | " + " | ".join(["c"] * (num_cols - 2)) + "|c|" # first column separate

    # Adjust column format to add a vertical line before the last column
    # column_format = "c | " + " | ".join(["c"] * (num_cols - 2)) + " | c"  # add vertical line before the last column

    lines = []
    lines.append("\\begin{table*}[tb]")
    lines.append("\\centering")
    lines.append(f"\\caption{{{caption}}}")
    lines.append(f"\\label{{{label}}}")
    lines.append(f"\\begin{{tabular}}{{{column_format}}}")
    lines.append("\\hline")

    # First row: expects something like ["CPU", ("Prover Time", 3), ("Verifier Time", 1)]
    first_row = array[0]
    first_cell = first_row[0]
    first_multicol_cells = first_row[1:]
    
    first_line = str(first_cell) + " & " + " & ".join(
        f"\\multicolumn{{{span}}}{{|c|}}{{{text}}}" for text, span in first_multicol_cells
    )
    lines.append(first_line + " \\\\")
    
    # Add the line separator between Prover Time and Verifier Time (vertical line between columns 2 and 3)
    lines.append("\\cline{2-5}") 

    # Second row: expects something like ["", ("Pre-process", 2), ("Post-process", 1)]
    second_row = array[1]
    second_cell = second_row[0]
    second_multicol_cells = second_row[1:]
    
    second_line = str(second_cell) + " & " + " & ".join(
        f"\\multicolumn{{{span}}}{{|c|}}{{{text}}}" for text, span in second_multicol_cells
    )  + " & "
    lines.append(second_line + " \\\\")
    lines.append("\\hline")

    # Remaining data rows
    for row in array[2:]:
        row_line = " & ".join(str(cell) for cell in row)
        lines.append(row_line + " \\\\")

    lines.append("\\hline")
    lines.append("\\end{tabular}")
    lines.append("\\end{table*}")
    
    return "\n".join(lines)


data = [
    ["CPU and hash function", ("Prover time", 4), ("Total verifier time", 1)],   # First row: multicolumns start from column 2
    ["", ("Pre-process",2), ("During communication",1),("Total",1)],
    ["", "Derive $(t,B)$", "Non-revocation", "Derive $H$", "", ""],       # Second row: subheaders
]

def get_title(array,  line):
    array.append(line.strip())


def get_prover_times_multcolumn(array, line, total_prover_time):
    value = line.strip()
    if value.endswith("ms"):
        num = value
    else: # second  
        num = float(value.replace("s", ""))
        num = num * 1000
        num = f"{num:.2f}ms"

    time = float(num.replace("ms", ""))
    array.append(f"{time:.2f}ms")
    total_prover_time += time
    return total_prover_time

def get_total_verifier_time(array, line):
    time_values = line.split(" + ")
    total_time = 0
    for time in time_values:
        value = time.strip()
        if value.endswith("ms"):
            num = value
        else: # seconds  
            num = float(value.replace("s", ""))
            num = num * 1000
            num = f"{num:.2f}ms"
        total_time += float(num.replace("ms", ""))
    formatted_total_time = f"{total_time:.2f}ms"
    
    array.append(formatted_total_time)

def get_total_prover_time(array, total_prover_time):
    array.insert(-1,f"{total_prover_time:.2f}ms")


#assuming all times are in ms
amd_sha =[]
with open('amd_sha.txt', 'r') as file:
    lines = file.readlines()
    total_prover_time=0
    for index, line in enumerate(lines):
        if index == 0:
            get_title(amd_sha, line)
        if index != len(lines) - 1 and index != 0:
            total_prover_time = get_prover_times_multcolumn(amd_sha, line, total_prover_time)
        if index == len(lines) - 1:
            get_total_verifier_time(amd_sha, line)

    # add the total prover time to the second last position (after calculation)
    get_total_prover_time(amd_sha, total_prover_time)

amd_poseidon =[]    
with open('amd_poseidon.txt', 'r') as file:
    lines = file.readlines()
    total_prover_time=0
    for index, line in enumerate(lines):
        if index == 0:
            get_title(amd_poseidon, line)
        if index != len(lines) - 1 and index != 0:
            total_prover_time = get_prover_times_multcolumn(amd_poseidon, line, total_prover_time)
        if index == len(lines) - 1:
            get_total_verifier_time(amd_poseidon, line)

    # add the total prover time to the second last position (after calculation)
    get_total_prover_time(amd_poseidon, total_prover_time)


apple_sha =[]
with open('apple_sha.txt', 'r') as file:
    lines = file.readlines()
    total_prover_time=0
    for index, line in enumerate(lines):
        if index == 0:
            get_title(apple_sha, line)
        if index != len(lines) - 1 and index != 0:
            total_prover_time = get_prover_times_multcolumn(apple_sha, line, total_prover_time)
        if index == len(lines) - 1:
            get_total_verifier_time(apple_sha, line)

    # add the total prover time to the second last position (after calculation)
    get_total_prover_time(apple_sha, total_prover_time)

apple_poseidon =[]
with open('apple_poseidon.txt', 'r') as file:
    lines = file.readlines()
    total_prover_time=0
    for index, line in enumerate(lines):
        if index == 0:
            get_title(apple_poseidon, line)
        if index != len(lines) - 1 and index != 0:
            total_prover_time = get_prover_times_multcolumn(apple_poseidon, line, total_prover_time)
        if index == len(lines) - 1:
            get_total_verifier_time(apple_poseidon, line)

    # add the total prover time to the second last position (after calculation)
    get_total_prover_time(apple_poseidon, total_prover_time)

data_sha = data + [amd_sha] + [apple_sha]
data_poseidon = [amd_poseidon] + [apple_poseidon]

# # print(data_sha)
print(array_to_latex_multicol_table(data_sha+data_poseidon, caption="Benchmark results with SHA-256 and Poseidon.", label="tab:benchmark"))
# print(array_to_latex_multicol_table(data_poseidon, caption="Benchmark results with Poseidon", label="tab:Poseidon"))