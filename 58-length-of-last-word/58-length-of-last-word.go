func lengthOfLastWord(s string) int {
    l := len(s)
    var j int
    for j = l - 1; j > -1; j-- {
        if s[j] != ' ' {
            break;
        }
    }
    for i := j; i > -1; i-- {
        if s[i] == ' ' {
            return j - i;
        }
    }
    return j + 1
}

// my testcases
// "haha "
// " hehe"
// " hoho "
// "neko"
