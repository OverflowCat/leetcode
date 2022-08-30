

char repeatedCharacter(char * s){
    bool map[26] = {false};
    for (unsigned i = 0; ; i++) {
        unsigned short id = s[i] - 'a';
        if (map[id]) return s[i];
        else map[id] = true;
    }
}
