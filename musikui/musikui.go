package musikui

import "fmt"

type Problem struct {
	a int
	b int
}

func NewProblem(a, b int) Problem {
	return Problem{a, b}
}

func (p Problem) Solve() (Answer, error) {
	for x := 0; x < 10; x++ {
		for y := 0; y < 10; y++ {
			left := y * (10*x + y)
			right := 100*p.a + 10*x + p.b
			if left == right {
				return NewAnswer(x, y), nil
			}
		}
	}
	return Answer{}, fmt.Errorf("failed to find answer")
}

type Answer struct {
	x int
	y int
}

func NewAnswer(x, y int) Answer {
	return Answer{x, y}
}
