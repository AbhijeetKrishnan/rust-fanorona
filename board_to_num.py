def board_to_hex(board):
    binary_string = ''.join([''.join(map(str, reversed(row))) for row in board])
    num = int(binary_string, 2)
    return num

def flipped_board_to_hex(board):
    binary_string = ''.join([''.join(map(str, row)) for row in board])[::-1]
    num = int(binary_string, 2)
    return num

def print_BB_for_rust(bb):
    print('\n'.join(["BitBoard(0x{:x}),".format(val) for val in bb]))

EMPTY = board_to_hex([
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
])

BB_A = board_to_hex([
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
])

WHITE = board_to_hex([
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
])

BLACK = board_to_hex([
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
])

BB_POS = [ 
    BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1, BB_I1,
    BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2, BB_I2,
    BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3, BB_I3,
    BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4, BB_I4,
    BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5, BB_I5,
] = [1 << i for i in range(45)]

BB_MOVES = [
    BB_A2 | BB_B1 | BB_B2,
    BB_A1 | BB_B2 | BB_C1,
    BB_B1 | BB_B2 | BB_C2 | BB_D1 | BB_D2,
    BB_C1 | BB_D2 | BB_E1,
    BB_D1 | BB_D2 | BB_E2 | BB_F1 | BB_F2,
    BB_E1 | BB_F2 | BB_G1,
    BB_F1 | BB_F2 | BB_G2 | BB_H1 | BB_H2,
    BB_G1 | BB_H2 | BB_I1,
    BB_H1 | BB_H2 | BB_I2,

    BB_A1 | BB_A3 | BB_B2,
    BB_A1 | BB_A2 | BB_A3 | BB_B1 | BB_B3 | BB_C1 | BB_C2 | BB_C3,
    BB_B2 | BB_C1 | BB_C3 | BB_D2,
    BB_C1 | BB_C2 | BB_C3 | BB_D1 | BB_D3 | BB_E1 | BB_E2 | BB_E3,
    BB_D2 | BB_E1 | BB_E3 | BB_F2,
    BB_E1 | BB_E2 | BB_E3 | BB_F1 | BB_F3 | BB_G1 | BB_G2 | BB_G3,
    BB_F2 | BB_G1 | BB_G3 | BB_H2,
    BB_G1 | BB_G2 | BB_G3 | BB_H1 | BB_H3 | BB_I1 | BB_I2 | BB_I3,
    BB_H2 | BB_I1 | BB_I3,

    BB_A2 | BB_A4 | BB_B2 | BB_B3 | BB_B4,
    BB_A3 | BB_B2 | BB_B4 | BB_C3,
    BB_B2 | BB_B3 | BB_B4 | BB_C2 | BB_C4 | BB_D2 | BB_D3 | BB_D4,
    BB_C3 | BB_D2 | BB_D4 | BB_E3,
    BB_D2 | BB_D3 | BB_D4 | BB_E2 | BB_E4 | BB_F2 | BB_F3 | BB_F4,
    BB_E3 | BB_F2 | BB_F4 | BB_G3,
    BB_F2 | BB_F3 | BB_F4 | BB_G2 | BB_G4 | BB_H2 | BB_H3 | BB_H4,
    BB_G3 | BB_H2 | BB_H4 | BB_I3,
    BB_H2 | BB_H3 | BB_H4 | BB_I2 | BB_I4,

    BB_A3 | BB_A5 | BB_B4,
    BB_A3 | BB_A4 | BB_A5 | BB_B3 | BB_B5 | BB_C3 | BB_C4 | BB_C5,
    BB_B4 | BB_C3 | BB_C5 | BB_D4,
    BB_C3 | BB_C4 | BB_C5 | BB_D3 | BB_D5 | BB_E3 | BB_E4 | BB_E5,
    BB_D4 | BB_E3 | BB_E5 | BB_F4,
    BB_E3 | BB_E4 | BB_E5 | BB_F3 | BB_F5 | BB_G3 | BB_G4 | BB_G5,
    BB_F4 | BB_G3 | BB_G5 | BB_H4,
    BB_G3 | BB_G4 | BB_G5 | BB_H3 | BB_H5 | BB_I3 | BB_I4 | BB_I5,
    BB_H4 | BB_I3 | BB_I5,

    BB_A4 | BB_B4 | BB_B5,
    BB_A5 | BB_B4 | BB_C5,
    BB_B4 | BB_B5 | BB_C5 | BB_D4 | BB_D5,
    BB_C5 | BB_D4 | BB_E5,
    BB_D4 | BB_D5 | BB_E4 | BB_F4 | BB_F5,
    BB_E5 | BB_F4 | BB_G5,
    BB_F4 | BB_F5 | BB_G4 | BB_H4 | BB_H5,
    BB_G5 | BB_H4 | BB_I5,
    BB_H4 | BB_H5 | BB_I4,
]

BB_ROW = [
    BB_1,
    BB_2,
    BB_3,
    BB_4,
    BB_5,
] = [int('0b111_111_111', 2) << (9 * i) for i in range(5)]

BB_COL = [
    BB_A, BB_B, BB_C, BB_D, BB_E, BB_F, BB_G, BB_H, BB_I
] = [(BB_A << i) for i in range(9)]

BB_RAY = None

DEL = {
    0: (1, 0),
    1: (1, 1),
    2: (0, 1),
    3: (-1, 1),
    4: (-1, 0),
    5: (-1, -1),
    6: (0, -1),
    7: (1, -1),
}

def generate_BB_RAY():
    global BB_RAY
    BB_RAY = [[0] * 8 for _ in range(45)]
    for sq in range(45):
        for dir in range(8):
            board = [[0] * 9 for _ in range(5)]
            r, c = sq // 9, sq % 9
            dt = DEL[dir]
            r += dt[0]
            c += dt[1]
            while 0 <= r < 5 and 0 <= c < 9:
                board[r][c] = 1
                r += dt[0]
                c += dt[1]
            # print("(sq={}, dir={})".format(sq, dir))
            # print('\n'.join([''.join(map(str, row)) for row in board]))
            # print('---')
            BB_RAY[sq][dir] = flipped_board_to_hex(board) if flipped_board_to_hex(board) & BB_MOVES[sq] > 0 else 0


if __name__ == '__main__':
    # print('\n'.join(map(hex, BB_MOVES)))
    # print(board_to_hex(WHITE))
    # print(board_to_hex(BLACK))
    generate_BB_RAY()
    for sq in range(45):
        print("[{}],".format(', '.join([f'BitBoard(0x{ele:x})' for ele in BB_RAY[sq]])))