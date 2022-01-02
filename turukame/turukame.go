package turukame

import "fmt"

type Problem struct {
	LegSum  int
	HeadSum int
	TuruLeg int
	KameLeg int
}

type TuruKame struct {
	Turu int
	Kame int
}

func (p Problem) Solve() (TuruKame, error) {
	answers := []TuruKame{}
	for turu := 1; turu < int(p.HeadSum); turu++ {
		kame := p.HeadSum - turu
		legs := turu*int(p.TuruLeg) + kame*int(p.KameLeg)
		if legs == p.LegSum {
			answers = append(answers, TuruKame{turu, kame})
		}
	}
	if len(answers) == 1 {
		return answers[0], nil
	} else {
		return TuruKame{}, fmt.Errorf("miss")
	}
}
