import numpy as np
from math import sqrt 
is_vectorization_activated = True
slot_count = 16 #  4 64 
n_rows_image = int(sqrt(slot_count))
n_cols_image = n_rows_image
N = n_rows_image*n_cols_image
is_cipher = 1 
is_signed = 0
input_image = np.random.randint(0,10,(N)) 
output_image = np.zeros((n_rows_image*n_cols_image))
######################################
kernel = [[-1,0,1],[-2,0,2],[-1,0,1]]
for x in range(n_rows_image): 
    for y in range(n_cols_image):
        t=0 
        for i in range(-1,2):
            for j in range(-1,2):
                ni = x + i;
                nj = y + j;
                if ni >= 0 and ni < n_rows_image and nj >= 0 and nj < n_cols_image :
                    t += kernel[i+1][j+1]*input_image[ni*n_cols_image+nj]
        output_image[(x*n_cols_image+y)%N]=t
###############################################################################
###############################################################################
if is_vectorization_activated :
    function_slot_count = 1 
    nb_inputs = N
    nb_outputs = N
    with open("fhe_io_example.txt", "w") as f:
        header = str(function_slot_count)+" "+str(nb_inputs)+" "+str(nb_outputs)+"\n"
        f.write(header)
        input_lines = []
        output_lines = []
        for i in range(n_rows_image):
            for j in range(n_cols_image):
                input_line= "in_{}{}".format(i,j)+" "+str(is_cipher)+" "+str(is_signed)+" "+str(int(input_image[i*n_cols_image+j]))+"\n"
                input_lines.append(input_line)
                output_line= "out_{}{}".format(i,j)+" "+str(is_cipher)+" "+str(int(output_image[i*n_cols_image+j]))+"\n"
                output_lines.append(output_line)
        f.writelines(input_lines)
        f.writelines(output_lines)
####################################################################
else :
    function_slot_count= N 
    nb_inputs = 1
    nb_outputs = 1
    #nb_outputs = n_rows_out*n_cols_out
    #nb_outputs = 1 # 3*3*4 
    with open("fhe_io_example.txt", "w") as f:
        header = str(function_slot_count)+" "+str(nb_inputs)+" "+str(nb_outputs)+"\n"
        f.write(header)
        f.write(f"img {is_cipher} {is_signed} "+" ".join(f"{num}" for num in input_image )+"\n")
        f.write(f"result {is_cipher} "+" ".join(f"{int(num)}" for num in output_image)+"\n")



