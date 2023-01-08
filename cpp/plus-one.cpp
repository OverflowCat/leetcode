#include <vector>
#include <iostream>
using std::vector;

class Solution
{
public:
    vector<int> plusOne(vector<int> &digits)
    {
        bool flag = false;
        for (auto i = digits.size(); i != 0; --i)
        {
            digits[i] += flag ? 2 : 1;
            if (digits[i] > 9)
            {
                digits[i] -= 10;
                flag = true;
            }
            else
            {
                flag = false;
            }
        }
        if (flag)
            digits.insert(digits.begin(), 1);
        return digits;
    }
};
