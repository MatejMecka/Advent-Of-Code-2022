with open('../TestCases/text_example.in') as f:
    lines = f.readlines()
    tempSum = 0
    maxSum = 0
    sums = []
    for line in lines:
        if line == '\n':
            if tempSum > maxSum:
                maxSum = tempSum
            sums.append(tempSum)
            tempSum = 0
        else:
            num = int(line.replace('\n', '')) 
            tempSum += num

    print(maxSum)

    sums.sort()
    sums = sums[::-1]

    print(sums[0] + sums[1] + sums[2])
