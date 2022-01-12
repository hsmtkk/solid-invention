package goldfish

type Problem struct {
	Pois        uint
	PoiPower    uint
	FishWeights []uint
}

func (p *Problem) Solve() uint {
	var fishes uint = 0
	pois := p.Pois
	poiPower := p.PoiPower
	fishIndex := 0
	for {
		w := p.FishWeights[fishIndex]
		if poiPower > w {
			poiPower -= w
			fishes += 1
			fishIndex += 1
		} else {
			poiPower = p.PoiPower
			pois -= 1
			if pois <= 0 {
				break
			}
		}
	}
	return fishes
}
