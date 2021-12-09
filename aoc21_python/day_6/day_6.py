f = open("input06")
f = f.read().split(",")
#make ints
nums = [0] * 10
for i in f:
   nums[int(i)] += 1
print(nums)
#
for j in range(256):
    print(j)
    for i in range(10):
        if i==0 :
            nums[7] = nums[7] + nums[0]
            nums[9] = nums[0]
            nums[0] = 0
        else:
            nums[i-1] = nums[i]
            nums[i] = 0
    print(nums)

lanternfish = 0
for l in nums:
    lanternfish = lanternfish + l
print(lanternfish)
