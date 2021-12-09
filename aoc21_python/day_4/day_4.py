f = open("input")
f = f.read().splitlines()
bingonums = []
for i in f[0].split(","): bingonums.append(int(i))
print(bingonums)

for i in range(len(f)):
    print(f[i])
