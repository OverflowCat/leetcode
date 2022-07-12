bool isValidSudoku(char ** board, int boardSize, int * boardColSize) {
  for (int i = 0; i < 9; i++) {
    short int map[10] = {
      0
    };
    for (int j = 0; j < 9; j++) {
      if (board[i][j] == '.')
        continue;
      short int n = board[i][j] - '0';
      if (map[n] == 0) map[n] = 1;
      else return false;
    }
  }
  for (int i = 0; i < 9; i++) {
    short int map[10] = {
      0
    };
    for (int j = 0; j < 9; j++) {
      if (board[j][i] == '.')
        continue;
      short int n = board[j][i] - '0';
      if (map[n] == 0) map[n] = 1;
      else return false;
    }
  }
  for (int i = 0; i < 9; i++) {
    short int map[10] = {
      0
    };
    for (int j = 0; j < 9; j++) {
      if (board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3] == '.')
        continue;
      short int n = board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3] - '0';
      if (map[n] == 0) map[n] = 1;
      else return false;
    }
  }
  return true;
}
