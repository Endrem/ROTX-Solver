# rot_hits is used for tracking which rotation has the most dictionary matches.
rot_hits = [26]
# dictionary_list is a multi-dimensional array that will contain the dictionary word and it's array of 'distances'.
# The range goes up to 15 because that's the largest word within the ScrabbleDictionary that I've consistently used.
dictionary_list = [[] for i in range(15)]

# Set the minimum and maximum letter length of words that will be checked from the dictionary file.
min_letters = 2
max_letters = 6


# This function takes a dictionary location and loads it into dictionary_list, after calculating letter 'distances'.
def load_dictionary(dictionary_file):
    dictionary = open(dictionary_file, "r")
    for dictionary_line in dictionary:
        for dictionary_word in dictionary_line.split():
            if (len(dictionary_word) >= min_letters) and (len(dictionary_word) <= max_letters):
                dictionary_distance = []
                get_distance(dictionary_word, dictionary_distance)
                dictionary_word_array = [dictionary_word, dictionary_distance]
                # dictionary_list appends dictionary words and their 'distances'.
                # Example: [ [['the', [14,23]], ['and', [13,16]]] ]
                dictionary_list[(len(dictionary_word) - 1)].append(dictionary_word_array)
    dictionary.close()


# Calculates the unicode 'distance' between each letter and adds it to an array.
# For example, the word "and" becomes [13,16].  "A" to "N" distance is 13 while "N" to "D" distance is 16.
def get_distance(word, array):
    for i in range(len(word) - 1):
        letter1 = ord(word[i].lower())
        letter2 = ord(word[i + 1].lower())
        if letter1 <= letter2:
            num = (letter2 - letter1)
        else:
            num = (123 - letter1) + (letter2 - 97)
        array.append(num)


# This function takes a ciphered sentence and calculates the 'distances' for each word within it using get_distance().
def test_sentence(rot_phrase):
    distance_phrase = []
    for rot_word in str.split(rot_phrase):
        # Only grab alphabetic letters (to filter out punctuation).
        rot_word = ''.join([i for i in rot_word if i.isalpha()])
        new_phrase = []
        get_distance(rot_word, new_phrase)
        distance_phrase.append(new_phrase)

    # Now, it compares each word from the phrase (as a 'distance' array) to the distance arrays in dictionary_list.
    for i in range(len(distance_phrase)):
        for dictionary_distance in dictionary_list[len(distance_phrase[i])]:
            if distance_phrase[i] == dictionary_distance[1]:
                rot = []
                # For calculating the rot, we take the first letter from the dictionary word and from the sentence word.
                # We then combine both letters as a "word" and we are returned the rot/distance between them.
                get_distance((dictionary_distance[0][0], rot_phrase.split()[i][0]), rot)
                # Lastly, we add a rot_hit to the rot number we were returned.
                rot_hits[rot[0]] += 1


# This is the function used to decipher text using a selected rot number.
def rot_decipher(text, rot):
    text = list(text)
    rot %= 26

    # This loop takes capitalization into account.  65-90 is A-Z and 97-122 is a-z.  Special characters are skipped.
    for i in range(len(text)):
        if 65 <= ord(text[i]) <= 90:
            if (ord(text[i]) + rot) > 90:
                text[i] = chr(ord(text[i]) + rot - 26)
            else:
                text[i] = chr(ord(text[i]) + rot)
        elif 97 <= ord(text[i]) <= 122:
            if (ord(text[i]) + rot) > 122:
                text[i] = chr(ord(text[i]) + rot - 26)
            else:
                text[i] = chr(ord(text[i]) + rot)
    return ''.join(text)


# Start of the main code.
load_dictionary("../Dictionary.txt")
cipher_file_in = open("../ROTSentences.txt", "r")
plaintext_file_out = open("Plaintext.txt", "w")

# Checking each line/sentence in the ciphered file.
for cipher_line in cipher_file_in:
    rot_hits = [[] for i in range(26)]
    for i in range(len(rot_hits)):
        rot_hits[i] = 0

    test_sentence(cipher_line)
    # This will decipher the sentence after retrieving the rot with the highest count.
    deciphered_line = rot_decipher(cipher_line, (-rot_hits.index(max(rot_hits))))
    plaintext_file_out.write(deciphered_line)


cipher_file_in.close()
plaintext_file_out.close()
