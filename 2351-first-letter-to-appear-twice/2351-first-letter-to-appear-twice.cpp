#include<bitset>

class Solution {
public:
    char repeatedCharacter(string s) {
        bitset<26> map;
        for (unsigned i = 0; ; i++) {
            unsigned short id = s[i] - 'a';
            if (map[id]) return s[i];
            else map[id] = true;
        }
    }
};
