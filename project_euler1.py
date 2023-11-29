import functools

# Constants
known_primes = {}
limit = 20_000

@functools.cache
def sieve(start=2, end=20_000):
    # Initialize a list
    primes = []
    for possiblePrime in range(start, end + 1):
        #  Assume number is prime until shown it is not.
        isPrime = True
        for num in range(2, int(possiblePrime ** 0.5) + 1):
            if possiblePrime % num == 0:
                isPrime = False
                break
        if isPrime:
            primes.append(possiblePrime)

    return primes


@functools.cache
def isprime(num):
    try:
        return known_primes[num]
    except:
        for n in range(2,int(num**0.5)+1):
            if num%n==0:
                known_primes[num] = False
                return False
        known_primes[num] = True
    return True


@functools.cache
def conc(num1, num2):
    return int(str(num1) + str(num2))


def find_comp_prime(values: list):
    for p in sieve(3, limit):
        eligible = True
        for v in values:
            if not isprime(conc(v, p)) or not isprime(conc(p,v)):
                eligible = False
        if eligible:
            return p
            # eligible_list.append(p)
    # return eligible_list


def main(k):
    smallest_sum = 9999999999999
    for p in sieve(3, limit):
        starting = [p]
        while len(starting) < k:
            try:
                eligible = find_comp_prime(starting)
                if isinstance(eligible, int):
                    starting.append(eligible)
                    print(starting)
                else:
                    break
            except Exception as e:
                break
        if len(starting) == k:
            if smallest_sum > sum(starting):
                smallest_sum = sum(starting)
                return starting, sum(starting)
        if smallest_sum < p:
            break


if __name__ == "__main__":
    _list, _main = main(5)
    print(_list, _main)
