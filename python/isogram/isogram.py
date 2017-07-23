def is_isogram(word):
    word = sorted(str.upper(word))
    i = 0

    while i < len(word)-1:
        if (word[i].isalpha()) & (word[i] == word[i+1]):
            return False
        else:
            i += 1

    return True
