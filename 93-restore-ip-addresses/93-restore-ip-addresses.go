func restoreIpAddresses(s string) []string {
	var res []string
	length := len(s)
	if length < 4 || length > 4*3 {
		return res
	}
	for i := 1; i < 4; i++ {
		for j := 1; j < 4; j++ {
			for k := 1; k < 4; k++ {
				l := length - i - j - k
				if l < 1 || l > 3 {
					continue
				}
				if i != 1 && s[0] == '0' {
					continue
				}
				a, err := strconv.ParseInt(s[0:i], 10, 32)
				if err != nil || a > 255 {
					continue
				}
				if j != 1 && s[i] == '0' {
					continue
				}
				b, err := strconv.ParseInt(s[i:i+j], 10, 32)
				if err != nil || b > 255 {
					continue
				}
				if k != 1 && s[i+j] == '0' {
					continue
				}
				c, err := strconv.ParseInt(s[i+j:i+j+k], 10, 32)
				if err != nil || c > 255 {
					continue
				}
				if l != 1 && s[i+j+k] == '0' {
					continue
				}
				d, err := strconv.ParseInt(s[i+j+k:], 10, 32)
				if err != nil || d > 255 {
					continue
				}
				r := s[0:i] + "." + s[i:i+j] + "." + s[i+j:i+j+k] + "." + s[i+j+k:]
				res = append(res, r)
			}
		}
	}
	return res
}
