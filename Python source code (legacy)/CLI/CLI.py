import os
from time import sleep
from random import randint


def cls():
    if os.name == 'nt':
        os.system('cls')
    else:
        os.system('clear')


def select_chars(selection, sc_kana, sc_romanised):
    char_list, romanised_char_list = [], []
    for column in selection.split(','):
        char_list.append(sc_kana[int(column) - 1].split(','))
        romanised_char_list.append(sc_romanised[int(column) - 1].split(','))
    chars = [char for element in char_list for char in element]
    romanised_chars = [char for element in romanised_char_list for char in element]
    print('Selected katakana characters in the following columns: ' + selection.replace(',', ', ') + '.')
    return chars, romanised_chars


chars_valid, h_chars_valid, k_chars_valid = False, False, False
h_char_list, romanised_h_char_list, k_char_list, romanised_k_char_list = [], [], [], []
hiragana = [line.rstrip('\n') for line in open('resources/hiragana_list')]
katakana = [line.rstrip('\n') for line in open('resources/katakana_list')]
romanised = [line.rstrip('\n') for line in open('resources/romanised_list')]

while not h_chars_valid or not k_chars_valid:
    cls()
    while not h_chars_valid:
        cls()
        print('Printing hiragana list.')
        print(open('resources/hiragana_full', 'r').read())
        h_selection = input("Select hiragana characters to test by typing below.  Enter column numbers separated by "
                            "commas (e.g. '1,2,4,10,26') to\nselect the kana in those columns.  You can also type "
                            "'all' or 'none' to make your selection.\n\nSelection: ")
        if h_selection == 'all':
            h_chars = [char for element in hiragana for char in element.split(',')]
            romanised_h_chars = [char for element in romanised for char in element.split(',')]
            print('Selected all hiragana characters.')
            h_chars_valid = True
        elif h_selection == 'none':
            print('No hiragana characters selected.')
            h_chars, romanised_h_chars = [], []
            h_chars_valid = True
        else:
            try:
                h_chars, romanised_h_chars = select_chars(h_selection, hiragana, romanised)
                print('Selected hiragana characters in the following columns: ' + h_selection.replace(',', ', ') + '.')
                h_chars_valid = True

            except (ValueError, IndexError):
                print('\nInvalid Selection.  Press Enter to try again.')
                input('')

    while not k_chars_valid:
        cls()
        print('\nPrinting katakana list.\n')
        print(open('resources/katakana_full', 'r').read())
        k_selection = input("Select katakana characters to test by typing below.  Enter column numbers separated by "
                            "commas (e.g. '1,2,4,10,26') to\nselect the kana in those columns.  You can also type "
                            "'all' or 'none' to make your selection.\n\nSelection: ")
        if k_selection == 'all':
            k_chars = [char for element in katakana for char in element.split(',')]
            romanised_k_chars = [char for element in romanised for char in element.split(',')]
            print('Selected all katakana characters.')
            k_chars_valid = True
        elif k_selection == 'none':
            print('No katakana characters selected.')
            k_chars, romanised_k_chars = [], []
            k_chars_valid = True
        else:
            try:
                k_chars, romanised_k_chars = select_chars(k_selection, katakana, romanised)
                k_chars_valid = True

            except (ValueError, IndexError):
                print('\nInvalid Selection.  Press Enter to try again.')
                input('')
    if h_selection == 'none' and k_selection == 'none':
        h_chars_valid, k_chars_valid = False, False
        print('\nNo kana selected.  Restarting.\n')
        sleep(2)
        continue

cls()
run, new_prompt, vowel = True, True, False
kana_num, previous_num = 0, -1
kana_list = h_chars + k_chars
romanised_list = romanised_h_chars + romanised_k_chars

while run:
    if new_prompt:
        kana_num = randint(0, len(kana_list) - 1)
        if kana_num == previous_num:
            continue
    print('Kana: ' + kana_list[kana_num])
    if romanised_list[kana_num] in 'aiueo':
        vowel = True
    response = input('Rōmaji: ')
    if response == 'exit':
        exit()
    elif response == 'skip':
        print("Answer was: '" + romanised_list[kana_num] + "'.\n")
        new_prompt = True
    elif not vowel and response in romanised_list[kana_num]:
        print('Correct!\n')
        new_prompt = True
    elif vowel and response == romanised_list[kana_num]:
        print('Correct!\n')
        new_prompt = True
    else:
        print('Incorrect.  Please try again.\n')
        new_prompt = False
    previous_num = kana_num
