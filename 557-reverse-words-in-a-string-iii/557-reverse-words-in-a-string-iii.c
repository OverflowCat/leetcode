

char * reverseWords(char * s){
    unsigned i = 0, ini = 0;
    while(true) {
        char c = s[i];
        if (c == '\0') break;
        if (c == ' ') {
            unsigned mid = (i + ini)/2;
            unsigned k = i - 1;
            for (unsigned j = ini; j < mid; j++) {
                char temp = s[j];
                s[j] = s[k];
                s[k--] = temp;
            }
            ini = i + 1;
        }
        i++;
    }
    
    unsigned mid = (i + ini)/2;
    unsigned k = i - 1;
    for (unsigned j = ini; j < mid; j++) {
        char temp = s[j];
        s[j] = s[k];
        s[k--] = temp;
    }
    ini = i + 1;
    return s;
}