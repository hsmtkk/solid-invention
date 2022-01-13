package robotprogram

type Request struct {
	Number int
	Action string
}

type Response struct {
	isNumber bool
	number   int
	actions  []string
}

func NewResponseNumber(number int) Response {
	return Response{isNumber: true, number: number, actions: nil}
}

func NewResponseActions(actions []string) Response {
	return Response{isNumber: false, number: 0, actions: actions}
}

func Process(end int, requests []Request) []Response {
	resps := []Response{}
	for i := 1; i <= end; i++ {
		actions := []string{}
		for _, req := range requests {
			if i%req.Number == 0 {
				actions = append(actions, req.Action)
			}
		}
		if len(actions) == 0 {
			resps = append(resps, NewResponseNumber(i))
		} else {
			resps = append(resps, NewResponseActions(actions))
		}
	}
	return resps
}
