import tkinter

vowels = 'aiueo'
romanised_list = [line.rstrip('\n') for line in open('resources/romanised')] * 2


def kana_data():
    kd_roman = 'su'
    image_path = 'resources/py.gif'
    return kd_roman, tkinter.PhotoImage(file=image_path)


class PromptAndPoll:
    def __init__(self, parent):
        self.background = 'green'
        self.response, self.lastchar = '', ''
        self.roman, self.image_reference = kana_data()
        self.image = tkinter.Label(window, compound=tkinter.CENTER, image=self.image_reference)
        self.image.pack()
        self.label = tkinter.Label(parent, text='', font="Arial 30", fg='black', bg=self.background, width=10)
        self.label.pack()
        self.entry = tkinter.Entry(fg="black", bg=self.background, width=50)
        self.entry.pack()
        self.label.after(10, self.poll_entry)

    def poll_entry(self):
        self.response = self.entry.get()
        if self.response:
            self.lastchar = list(self.response)[len(self.response)-1]
            if self.lastchar in vowels:
                self.submit_response()
            elif self.roman == 'n' and self.lastchar == 'n':
                self.submit_response()
        self.label.configure(text=self.response)
        self.label.after(10, self.poll_entry)

    def submit_response(self):
        print('Submitting Response.')
        if self.response == self.roman:
            print('Response Correct.')
            self.label.config(bg='green')
            self.entry.config(bg='green')
        elif self.response != self.roman:
            print('Response Incorrect')
            self.label.config(bg='red')
            self.entry.config(bg='red')
        self.entry.delete(0, tkinter.END)


if __name__ == "__main__":
    window = tkinter.Tk()
    prompt = PromptAndPoll(window)
    window.mainloop()
