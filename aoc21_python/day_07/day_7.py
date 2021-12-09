def fuel_test(location, nums):
    fuelsum = 0
    for i in nums:
        x = abs(i-location)
        fuelsum += ((x)*(x+1))/2
    return fuelsum


f = open("input")
f = f.read().split(",")
#make ints
nums = []
for i in f:
   nums.append(int(i))
lowest = 10000000000;
for i in range(2000):
    temp = fuel_test(i, nums)
    if temp < lowest: lowest = temp
print(lowest)


