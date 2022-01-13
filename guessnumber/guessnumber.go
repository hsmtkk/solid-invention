package guessnumber

type Operator = int

const (
	LessThan Operator = iota
	GreaterThan
	DividedBy
)

type Condition struct {
	operator Operator
	operand  int
}

func NewCondition(operator Operator, operand int) *Condition {
	return &Condition{operator, operand}
}

func (c *Condition) satisfy(n int) bool {
	switch c.operator {
	case LessThan:
		return n < c.operand
	case GreaterThan:
		return n > c.operand
	case DividedBy:
		return n%c.operand == 0
	}
	return false
}

type Conditions struct {
	conditions []*Condition
}

func NewConditions(conditions []*Condition) *Conditions {
	return &Conditions{conditions}
}

func (cs *Conditions) satisfy(n int) bool {
	for _, c := range cs.conditions {
		if !c.satisfy(n) {
			return false
		}
	}
	return true
}

func (cs *Conditions) GuessNumber() int {
	for i := 1; i <= 100; i++ {
		if cs.satisfy(i) {
			return i
		}
	}
	return 0
}
