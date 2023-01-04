#include <iostream>
#include <vector>
#include <map>
class Solution
{
public:
    int minimumRounds(std::vector<int> &tasks)
    {
        std::map<int, uint32_t> hm;
        for (size_t i = 0; i < tasks.size(); i++)
        {
            auto count = hm.find(tasks[i]);
            if (count == hm.end())
                hm.insert({tasks[i], 1});
            else
                count->second += 1;
        }
        int ans = 0;
        for (auto const &[_, v] : hm)
        {
            if (v == 1)
                return -1;
            auto rem = v % 3;
            ans += v / 3;
            if (rem != 0)
                ans += 1;
        }
        return ans;
    }
};
