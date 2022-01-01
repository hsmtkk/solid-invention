package typhoonapproach

type Route []int

func NewRoute(rain ...int) Route {
	return rain
}

type Problem struct {
	Limit  int
	Routes []Route
}

func NewProblem(limit int, routes []Route) Problem {
	return Problem{limit, routes}
}

func Solve(problem Problem) (bool, []Route) {
	results := []Route{}
	for _, route := range problem.Routes {
		if canPass(problem.Limit, route) {
			results = append(results, route)
		}
	}
	if len(results) > 0 {
		return true, results
	} else {
		return false, nil
	}
}

func canPass(limit int, route Route) bool {
	for _, rain := range route {
		if limit <= rain {
			return false
		}
	}
	return true
}
