EMPTY = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
]

WHITE = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
]

BLACK = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
]

def board_to_num(board):
    binary_string = ''.join(reversed([''.join(map(str, row)) for row in board]))
    hex_repr = hex(int(binary_string, 2))
    return hex_repr

def get_bb_pos_table():
    lookup = [[0] * 9 for _ in range(5)]
    board = EMPTY
    for row in range(5):
        for col in range(9):
            board[row][col] = 1
            lookup[row][col] = board_to_num(board)
            board[row][col] = 0
    return lookup

def get_bb_row_table():
    bb_row = []
    board = EMPTY
    for row in range(5):
        board[row] = [1] * 9
        bb_row.append(board_to_num(board))
        board[row] = [0] * 9
    return bb_row

def get_bb_col_table():
    bb_col = []
    board = EMPTY
    for col in range(9):
        for row in range(5):
            board[row][col] = 1
        bb_col.append(board_to_num(board))
        for row in range(5):
            board[row][col] = 0
    return bb_col

if __name__ == '__main__':
    print(board_to_num(WHITE))
    print(board_to_num(BLACK))
    
    # lookup = get_bb_pos_table()
    for row in range(5):
        for col in range(9):
            col_letter = chr(ord('A') + col)
            print(f"const BB_{col_letter}{row + 1} = 1 << {row * 9 + col};")
    # print('\n'.join("[{}],".format(', '.join("BB_{}{}".format(chr(ord('A') + col), row) for col in range(9))) for row in range(5, 0, -1)))
    # print('\n'.join([', '.join(map(str, row)) for row in lookup]))
    # print(', '.join([f'{str(val)}' for val in get_bb_col_table()]))