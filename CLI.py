from random import randint

syllabary_valid = False

while not syllabary_valid:
    syllabary = input("Choose characters to display by typing 'hiragana', 'katakana', or 'both': ")
    print('\n')
    if syllabary == 'exit':
        exit()
    elif syllabary == 'both':
        kana_list = [line.rstrip('\n') for line in open('resources/hiragana')]\
                    + [line.rstrip('\n') for line in open('resources/katakana')]
        syllabary_valid = True
    elif syllabary == 'hiragana' or syllabary == 'katakana':
        kana_list = [line.rstrip('\n') for line in open('resources/' + syllabary)]
        syllabary_valid = True
    else:
        print('Selection invalid.\n')

run, new_prompt = True, True
romanised_list = [line.rstrip('\n') for line in open('resources/romanised')] * 2

while run:
    if new_prompt:
        kana_num = randint(0, len(kana_list) - 1)
    current_kana = kana_list[kana_num]
    print('Kana: ' + current_kana)
    search_input = input('Rōmaji: ')
    if search_input == 'exit':
        exit()
    elif search_input == 'skip':
        print("Answer was: '" + romanised_list[kana_num] + "'.\n")
        new_prompt = True
    elif search_input in romanised_list[kana_num]:
        print('Correct!\n')
        new_prompt = True
    else:
        print('Incorrect.  Please try again.\n')
        new_prompt = False
