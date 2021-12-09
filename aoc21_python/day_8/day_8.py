f = open("input")
f = f.read().splitlines()
inputt = []
output = []
for i in f:
    io = i.split("|")
    inputt.append(io[0])
    output.append(io[1])
for i in range(len(inputt)):
    a_to_g = [0] * 7
    lengths = [[] for z in range(8)]
    for j in inputt[i].split():
        #print("j: ", j, "len(j): ", len(j))
        lengths[len(j)].append(j)
    # FIND A
    cf = []
    for k in list(lengths[3][0]):
        cf = list(lengths[2][0])
        if k != cf[0] and k != cf[1]:
            #print("cf: " , cf , " k: ", k)
            a_to_g[0] = k
    # FIND C & F
    cde = []
    letters = ['a','b','c','d','e','f','g']
    for k in letters:
        if not k in list(lengths[6][0]): cde.append(k)
        if not k in list(lengths[6][1]): cde.append(k)
        if not k in list(lengths[6][2]): cde.append(k)
    #print(cde)
    for k in cde:
        if k == cf[0]:
            a_to_g[2] = cf[0]
            a_to_g[5] = cf[1]
        elif k == cf[1]:
            a_to_g[2] = cf[1]
            a_to_g[5] = cf[0]
    # FIND B & D
    bd = list(lengths[4][0])
    #print(bd)
    already = False
    for k in range(len(bd)):
        if already: k = k - 1
        #print(a_to_g)
        if bd[k] == a_to_g[2] or bd[k] == a_to_g[5] : 
            bd.pop(k)
            #print(k)
            if already: break 
            already = True
    #print("cf: ", cf)
    #print("cde: ", cde)
    #print(bd)
    for k in letters:
        if k in list(cde) and k in list(bd): a_to_g[3] = k 
        elif k in list(bd) and not k in list(cde): a_to_g[1] = k
    for k in letters:
        if k in list(lengths[5][0]) and not k in list(lengths[5][1]) and not k in list(lengths[5][2]) and not k in a_to_g: a_to_g[6] = k
        if k in list(lengths[5][1]) and not k in list(lengths[5][2]) and not k in list(lengths[5][0]) and not k in a_to_g: a_to_g[6] = k
        if k in list(lengths[5][2]) and not k in list(lengths[5][1]) and not k in list(lengths[5][0]) and not k in a_to_g: a_to_g[6] = k
    for k in letters: 
        if not k in a_to_g: a_to_g[4] = k
    print("A - G: ", a_to_g)

seven = 0
four = 0
one = 0
eight = 0
for i in output:
    digits = []
    for j in i.split():
        #print(a_to_g)
        #print(j)
        length = len(j)
        if    length == 3: 
            seven += 1
            digits.append(7)
        elif  length == 4: 
            four += 1
            digits.append(4)
        elif  length == 2: 
            one += 1  
            digits.append(1)
        elif  length == 7: 
            eight += 1
            digits.append(8)
        elif length == 6:
            if not a_to_g[3] in list(j): digits.append(0)
            elif not a_to_g[2] in list(j): digits.append(6)
            elif not a_to_g[4] in list(j): digits.append(9)
        elif length == 5:
            if not a_to_g[2] in list(j): digits.append(5)
            elif not a_to_g[4] in list(j): digits.append(3)
            else: digits.append(2)
        else:
            print(Problem)
    print(digits)
print("Part 1:" , seven+four+one+eight)

