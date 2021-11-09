#!/usr/bin/env python3
# This is a sample Python script.

# Press Ctrl+Alt+R to execute it or replace it with your code. Press Double
# Shift to search everywhere for classes, files, tool windows, actions,
# and settings.

def print_argument(func):
    def wrapper(the_number):
        print(f"Argument for {func.__name__} is {the_number}")
        return func(the_number)
    return wrapper


@print_argument
def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    if name == '':
        print("You didn't enter your name!")
    else:
        print("Hi there...")  # Press Ctrl+F8 to toggle the breakpoint.

    for letter in name:
        print(letter)


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    your_name = input("Enter your name:")
    print_hi(your_name)

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
