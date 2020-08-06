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
    # state = load_state_from_file('penta_decathlon')
    # state = load_state_from_file('glider')
    # state = init_state_random(80, 38)
    state = init_state_random(1_000, 1_000)

    # draw(state)
    print('Tick 1 !')

    for _ in range(limit):
        time.sleep(wait)
        state = gen_next(state)
        # draw(state)
        print('Tick !')


def native_rust(width, height, limit, wait, debug=False):
    import ctypes
    from ctypes import cdll

    lib = cdll.LoadLibrary("target/debug/liblife.so")

    lib.init_state_random.argtypes = (ctypes.c_int32, ctypes.c_int32, ctypes.c_int32)
    lib.init_state_random.restype = ctypes.c_void_p

    # next_state
    lib.next_state.argtypes = (ctypes.c_void_p,)
    lib.next_state.restype = ctypes.c_void_p
    # free_char_p
    lib.free_char_p.argtypes = (ctypes.c_void_p,)
    # free_void_p
    lib.free_void_p.argtypes = (ctypes.c_void_p,)

    state = None
    txt_ptr = None

    try:
        state_ptr = lib.init_state_random(width, height, 4)

        # TODO: print init state

        for i in range(limit):
            time.sleep(wait)

            # print('py:', '{:x}'.format(state_ptr), type(state_ptr))
            # print('py ctype:', state_ptr, ctypes.c_void_p(state_ptr), ctypes.c_void_p(state_ptr).value)
            # print('py2:', '{:x}'.format(state_ptr), type(state_ptr))
            txt_ptr = lib.next_state(state_ptr)
            txt = ctypes.cast(txt_ptr, ctypes.c_char_p).value.decode('utf-8')
            # print('py: txt', '{:x}'.format(txt_ptr), type(txt_ptr))
            lib.free_char_p(txt_ptr)
            txt_ptr = None
            # try:
            # except Exception:
            #     txt = 'ERROR'
            print(txt)
            # finally:
            #     lib.theme_song_free(ptr)

        lib.free_void_p(state_ptr)
        state_ptr = None
        print("done python ctypes!")

    except KeyboardInterrupt:
        # raise
        pass
    finally:
        lib.free_char_p(txt_ptr)
        lib.free_void_p(state_ptr)
        quit()


    # from cffi import FFI


    # ffi = FFI()
    # lib = ffi.dlopen("target/debug/liblife.so")

    # ffi.cdef('void *init_state_random(int, int, int);')
    # ffi.cdef('void next_state(void *state_ptr);')

    # state = lib.init_state_random(10, 5, 2)
    # # print('py:', '{:x}'.format(state), type(state))
    # for i in range(10):
    #     print('py:', lib.next_state(state))

    # print("done python cffi!")


if __name__ == '__main__':
    # life(100_000, wait=0.06)
    # life(100_000, wait=0.120)
    # life(100_000, wait=0.6)

    native_rust(
        width=80, height=38,
        limit=10_000,
        wait=0.120,
        debug=True)
