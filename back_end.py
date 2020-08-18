from random import randint

syllabary_valid = False
while not syllabary_valid:
    syllabary = input("Choose characters to display by typing 'hiragana', 'katakana', or 'both': ")
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
        print('Selection invalid.')

run, new_prompt = True, True
romanised_list = [line.rstrip('\n') for line in open('resources/romanised')]

while run:
    if new_prompt:
        num = randint(0, len(kana_list) - 1)
    current_kana = kana_list[num]
    print(current_kana)
    search_input = input('Rōmaji: ')
    if search_input == 'exit':
        exit()
    elif search_input == 'skip':
        new_prompt = True
    elif search_input == romanised_list[num]:
        print('Correct!')
        new_prompt = True
    else:
        print('Incorrect.  Please try again.')
        new_prompt = False
