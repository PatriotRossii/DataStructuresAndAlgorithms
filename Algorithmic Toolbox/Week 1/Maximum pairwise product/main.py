def maximum_product(data):
    result = 0
    length = len(data)
    for i in range(length):
        for j in range(i + 1, length):
            result = max(result, data[i] * data[j])
    return result


data = map(int, input().split())
print(maximum_product(list(data)))