int climbStairs(int n){
    if (n < 4) return n;
    int a = 1, b = 2;
    for (unsigned short x = 3; x <= n; x++) {
        if (a > b) b += a;
        else a += b;
        // printf("a %d, b %d\n", a, b);
    }
    return a > b ? a : b;
}