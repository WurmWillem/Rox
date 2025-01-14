def fib(n):
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

def fib2(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


def fact(n):
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result

print(fib2(100))

