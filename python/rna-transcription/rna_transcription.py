def to_rna(dna):
    out = ""

    for x in dna:
        if x == 'G':
            out += 'C'
        elif x == 'C':
            out += 'G'
        elif x == 'T':
            out += 'A'
        elif x == 'A':
            out += 'U'
        else:
            return ''

    return out
