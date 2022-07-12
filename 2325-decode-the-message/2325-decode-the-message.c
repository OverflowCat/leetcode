

char * decodeMessage(char * key, char * message){
    char map[26];
    for (int i = 0; i < 26; i++) {
        map[i] = '1';
    }
    {
        char alpha = 'a';
        unsigned i = 0;
        while (true) {
            char c = key[i++];
            if (c == 32) continue;
            if ('\0' == c) break;
            unsigned posi = c - 'a';
            if (map[posi] != '1') continue;
            map[posi] = alpha;
            if (alpha++ == 'z') break;
        }
    }
    {
        unsigned i = 0;
        while (true) {
            char c = message[i];
            if ('\0' == c) break;
            if (' ' == c) {
                i++;
                continue;
            };
            message[i] = map[c-'a'];
            i++;
        }
    }
    return message;
}