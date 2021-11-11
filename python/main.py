#!/usr/bin/env pipenv-shebang
# This is a sample Python script.

# Press Ctrl+Alt+R to execute it or replace it with your code. Press Double
# Shift to search everywhere for classes, files, tool windows, actions,
# and settings.

import threading
import time


# An I/O intensive calculation.
# We simulate it with sleep.
def heavy(n, my_id):
    time.sleep(n)
    print(my_id, "is done")


def threaded(n):
    threads = []

    for i in range(n):
        t = threading.Thread(target=heavy, args=(2, i,))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()


if __name__ == "__main__":
    start = time.time()
    threaded(80)
    end = time.time()
    print("Took: ", end - start)
