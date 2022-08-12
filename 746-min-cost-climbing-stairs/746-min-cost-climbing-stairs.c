int minCostClimbingStairs(int* cost, int costSize){
    int total[1002];
    total[0] = 0;
    total[1] = 0;
    int end = costSize + 1;
    for (int i = 2; i < end; i++) {
        int a = total[i-1] + cost[i-1];
        int b = total[i-2] + cost[i-2];
        total[i] = a < b ? a : b;
    }
    return total[costSize];
}
