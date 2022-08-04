
# 
# format: '1-3 a: abcdefg', '10-13 b: asdfasdiv'

import time

t0 = time.perf_counter()

valid_password_count = 0

with open ('input.txt') as f:
    data = f.read()
   

data = data.splitlines()
for entry in data:
    min_num = int(entry.split('-')[0])
    max_num = int(entry.split('-')[1].split(': ')[0].split(' ')[0])
    char = entry.split('-')[1].split(': ')[0].split(' ')[1]
    password = entry.split(': ')[1]
    #print(min_num, max_num, password, char)
    
    if char not in password:
        continue
    else:
        #Part 1
        #i = 0
        #for c in password:
        #    if c == char:
        #        i += 1
        #if i <= max_num and i >= min_num:
        #    valid_password_count += 1
        #else:
        #    continue
        
        if (password[min_num-1] == char and password[max_num-1] != char 
                or password[min_num-1] != char and password[max_num-1] == char):
            valid_password_count += 1

print(valid_password_count)
t_end = time.perf_counter() - t0
print(f'time: {t_end}')
