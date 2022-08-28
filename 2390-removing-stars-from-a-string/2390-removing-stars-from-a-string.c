

char * removeStars(char * s){
    unsigned i = 0, j = 0;
    while(1) {
        char c = s[i];
        if (c == '\0') break;
        if (c == '*') j--; else s[j++] = s[i];
        i++;
    }
    s[j] = '\0';
    return s;
}
