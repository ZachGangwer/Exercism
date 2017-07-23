def distance(first, second):
    count = 0;
    i = 0;

    if len(first) != len(second):
        raise ValueError("Lengths differ: {} != {}", len(first), len(second));

    while i < len(first):
        if first[i] != second[i]:
            count += 1;

        i += 1;

    return count;
