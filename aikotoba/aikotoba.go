package aikotoba

type Authenticator struct {
	secret string
}

func New(secret string) *Authenticator {
	return &Authenticator{secret}
}

func (a *Authenticator) Authenticate(word string) bool {
	if a.secret == word {
		return false
	}
	c0 := charCounts(a.secret)
	c1 := charCounts(word)
	return compareCounter(c0, c1)
}

func charCounts(word string) map[rune]uint {
	counter := map[rune]uint{}
	for _, r := range word {
		count, ok := counter[r]
		if ok {
			counter[r] = count + 1
		} else {
			counter[r] = 1
		}
	}
	return counter
}

func compareCounter(c0, c1 map[rune]uint) bool {
	if len(c0) != len(c1) {
		return false
	}
	for k0, v0 := range c0 {
		v1, ok := c1[k0]
		if ok {
			if v0 != v1 {
				return false
			}
		} else {
			return false
		}
	}
	return true
}
