def load_memory(input_file):
    with open(input_file) as f:
        for line in f:
            return list(map(int, line.split(',')))

def compute(memory):
    print (memory)
    op, arg1, arg2, dest = memory[:4]
    if op == 99:
        return memory[0]
    elif op == 1:
        memory[dest] = arg1 + arg2
    elif op == 2:
        memory[dest] = arg1 * arg2


if __name__ == '__main__':
    memory = load_memory('./resources/2/test_input.txt')
    compute(memory)
