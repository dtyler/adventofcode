fuel = 0

def computeFuel(mass):
    fuel = int(int(mass) / 3) - 2
    if (fuel) < 0:
        return 0
    else:
        return fuel + computeFuel(fuel)

with open('./resources/1/input.txt', 'r') as f:
    for mass in f:
        fuel += computeFuel(mass)

print('fuel required %d', fuel)