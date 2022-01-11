package summervacation

type Span struct {
	begin uint
	end   uint
}

func NewSpan(begin, end uint) Span {
	return Span{begin, end}
}

func (s Span) Include(day uint) bool {
	return s.begin <= day && day <= s.end
}

type Solver struct {
}

func NewSolver() *Solver {
	return &Solver{}
}

func (s *Solver) Solve(spans []Span) bool {
	for d := 1; d <= 31; d++ {
		found := true
		for _, sp := range spans {
			if !sp.Include(uint(d)) {
				found = false
				break
			}
		}
		if found {
			return true
		}
	}
	return false
}
