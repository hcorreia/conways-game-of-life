#!/usr/bin/env python3
import random
import time


# LIVE = '*'
# DEAD = '.'

LIVE = 1
DEAD = 0

LIVE_CHAR = '\u2588\u2588'  # \u2588 OR \u2588
DEAD_CHAR = '\u2591\u2591'


def init_state_empty(width, height):
    return [[None]*width for _ in range(height)]


def init_state_random(width, height):
    return [[random.choice((LIVE, DEAD)) for i in range(width)] for j in range(height)]


def load_state_from_file(name):
    live = '*'
    dead = '.'
    state = []

    with open('state_{}.txt'.format(name), 'r') as fd:
        for line in fd.readlines():
            state.append([LIVE if c == live else DEAD for c in line])

    return state


def init_state(name=None):
    if name:
        return load_state_from_file(name)
    return init_state_random


def is_cell_alive(state, x, y, width, height):
    if x >= 0 and x < width and y >= 0 and y < height:
        return state[y][x] == LIVE
    return False


def gen_next(state):
    width = len(state[0])
    height = len(state)
    next_state = init_state_empty(width, height)

    for y in range(height):
        for x in range(width):
            neighbors = 0

            if is_cell_alive(state, x-1, y-1, width, height): neighbors += 1
            if is_cell_alive(state, x  , y-1, width, height): neighbors += 1
            if is_cell_alive(state, x+1, y-1, width, height): neighbors += 1
            if is_cell_alive(state, x+1, y  , width, height): neighbors += 1
            if is_cell_alive(state, x+1, y+1, width, height): neighbors += 1
            if is_cell_alive(state, x  , y+1, width, height): neighbors += 1
            if is_cell_alive(state, x-1, y+1, width, height): neighbors += 1
            if is_cell_alive(state, x-1, y  , width, height): neighbors += 1

            # Could be refactored changing init random from None to DEAD
            if state[y][x] == LIVE:
                if neighbors <= 1:
                    next_state[y][x] = DEAD
                elif neighbors >= 4:
                    next_state[y][x] = DEAD
                else:
                    next_state[y][x] = LIVE
            else:
                if neighbors == 3:
                    next_state[y][x] = LIVE
                else:
                    next_state[y][x] = DEAD

            # print('')
            # print('state[{}][{}] = {} :: {}'.format(x, y, state[y][x], neighbors))
            # print('next_[{}][{}] = {}'.format(x, y, next_state[y][x]))
            # print(next_state[y][x])

    # print(next_state)
    return next_state


last_screen = None


def draw(state):
    global last_screen

    # LOL
    txt = '\n'.join([''.join(LIVE_CHAR if c == LIVE else DEAD_CHAR for c in l) for l in state])

    if txt == last_screen:
        quit()

    last_screen = txt

    print('')
    print('')
    print('')
    print('')
    print(txt)



def life(limit, wait=1):
    # state = load_state_from_file('toad')
    # state = load_state_from_file('beacon')
    state = load_state_from_file('penta_decathlon')
    # state = load_state_from_file('glider')
    # state = init_state_random(80, 38)

    draw(state)

    for _ in range(limit):
        time.sleep(wait)
        state = gen_next(state)
        draw(state)


if __name__ == '__main__':
    # life(100000, wait=0.06)
    # life(100000, wait=0.120)
    life(100000, wait=0.6)
