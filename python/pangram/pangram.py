def is_pangram(word):
    word = sorted(word)
    i = 1
    count = 0

    while i < len(word):
        if (word[i] != word[i-1]) & (word[i].isalpha()):
            count += 1
        
        i += 1

    if count == 26:
        return True
    else: 
        return False
