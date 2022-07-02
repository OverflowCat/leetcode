unsigned long long twopown[39] = {0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824};

double myPow(double x, int m){
    double pool[39];
    long long n = m;
    if (n == 0) return 1;
    if (n == 1) return x;
    if (n < 0) {
        x = 1/x;
        n = -n;
    }
    pool[0] = 1;
    pool[1] = x;
    unsigned i = 2;
    while(twopown[i] <= n && i < 31) {
        pool[i] = pool[i - 1] * pool[i - 1];
        i++;
    }
    double res = pool[i-1];
    res *= myPow(x, n - twopown[i-1]);
    return res;
}
