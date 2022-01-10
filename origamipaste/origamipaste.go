package origamipaste

type Problem struct {
	Count    uint32
	Edge     uint32
	Overlaps []uint32
}

func (p *Problem) Solve() uint32 {
	return p.Edge * (p.Count*p.Edge - sum(p.Overlaps))
}

func sum(ns []uint32) uint32 {
	var s uint32 = 0
	for _, n := range ns {
		s += n
	}
	return s
}
