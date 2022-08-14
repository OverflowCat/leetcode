#include <math.h>
#include <stdio.h>

int countNumbersWithUniqueDigits(int n) {
    if (n == 0) {
        return 1;
    }
    if (n == 1) {
        return 10;
    }
    int ans = 10, cur = 9;
    for (int i = 0; i < n - 1; ++i) {
        cur *= 9 - i;
        ans += cur;
    }
    return ans;
}

int countSpecialNumbers(int n) {
  // if (n <= 9) {
  //   return n;
  // }
  // int dp[12] = {0};
  // dp[0] = 1; // 0
  // dp[1] = 9; // 0~9
  int weishu = 0;
  int _n = n;
  while (_n != 0) {
    _n /= 10;
    weishu++;
  }
  // for (int i = 2; i < weishu; i++) {
  //   dp[i] = dp[i - 1] + (dp[i - 1] - dp[i - 2]) * (10 - (i - 1));
  // }
  // int sum = 0;
  // for (int i = 1; i < weishu; i++) {
  //   sum += dp[i];
  // }
  int sum = countNumbersWithUniqueDigits(weishu - 1) - 1;
  
  for (int j = pow(10, weishu - 1); j <= n; j++) {
    short int is_used[10] = {0};
    // printf("j is %d\n", j);
    int _j = j;
    int flag = 1;
    while (_j != 0) {
      int yushu = _j % 10;
      // printf("%d ", yushu);
      if (is_used[yushu]++ == 1) {
        flag = 0;
        // printf("!!!! j is %d\n", j);
        break;
      }
      _j /= 10;
    }
    if (flag == 1)
      sum += 1;
  }
  return sum;
}

int main(void) {
  int res = countSpecialNumbers(20);
  printf("%d\n", res);
  return 0;
}
