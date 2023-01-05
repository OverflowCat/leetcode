#include <vector>
#include <iostream>
using std::vector;

class Solution
{
public:
    vector<int> plusOne(vector<int> &digits)
    {
        bool flag = true;
        for (auto i = digits.size() - 1; i != -1; --i)
        {
            if (digits[i] == 9)
            {
                digits[i] = 0;
            }
            else
            {
                digits[i] += 1;
                flag = false;
                break;
            }
        }
        if (flag) digits.insert(digits.begin(), 1);
        return digits;
    }
};
