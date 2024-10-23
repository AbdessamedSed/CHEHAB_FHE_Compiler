import numpy as np
function_slot_count= 16
is_vectorization_activated = True
c1 = np.random.randint(0,10,(function_slot_count))
c2 = np.random.randint(0,10,(function_slot_count))
result = np.zeros((function_slot_count)) 
res = 0 
for i in range(function_slot_count): 
    res+= (c1[i] - c2[i])*(c1[i] - c2[i]) 
for i in range(function_slot_count):
    result[i]=res 

is_cipher = 1 
is_signed = 1
if is_vectorization_activated :
    with open("fhe_io_example.txt","w") as file : 
        nb_inputs= 2*function_slot_count
        nb_outputs = 1
        slot_count = 1
        header = str(slot_count)+" "+str(nb_inputs)+" "+str(nb_outputs)+"\n"
        file.write(header) 
        rows=[]
        for i in range(function_slot_count):
            row = "c1_{}".format(i)+" "+str(is_cipher)+" "+str(is_signed)+" "+str(c1[i])+"\n"
            rows.append(row)
        for i in range(function_slot_count):
            row = "c2_{}".format(i)+" "+str(is_cipher)+" "+str(is_signed)+" "+str(c2[i])+"\n"
            rows.append(row)
        row = "result"+" "+str(is_cipher)+" "+str(int(result[i]))+"\n"
        rows.append(row)
        file.writelines(rows)
            

else :
    with open("fhe_io_example.txt","w") as file : 
        nb_inputs= 2
        nb_outputs = 1
        header = str(function_slot_count)+" "+str(nb_inputs)+" "+str(nb_outputs)+"\n"
        file.write(header) 
        rows=[]
        row = "c1"+" "+str(is_cipher)+" "+str(is_signed)+" "+" ".join(str(num) for num in c1)+"\n"
        rows.append(row)
        row = "c2"+" "+str(is_cipher)+" "+str(is_signed)+" "+" ".join(str(num) for num in c2)+"\n"
        rows.append(row)
        row = "result"+" "+str(is_cipher)+" "+" ".join(str(int(num)) for num in result)+"\n"
        rows.append(row)
        file.writelines(rows)