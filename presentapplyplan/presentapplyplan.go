package presentapplyplan

type Problem struct {
	N int
	X int
	Y int
}

type Present int

const (
	A Present = iota
	B
	AB
	N
)

func Solve(p Problem) []Present {
	result := []Present{}
	for i := 1; i <= p.N; i++ {
		a := i%p.X == 0
		b := i%p.Y == 0
		var present Present
		if a && b {
			present = AB
		} else if a {
			present = A
		} else if b {
			present = B
		} else {
			present = N
		}
		result = append(result, present)
	}
	return result
}
