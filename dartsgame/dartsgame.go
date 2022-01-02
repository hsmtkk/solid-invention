package dartsgame

import (
	"fmt"
	"math"
)

type Darts struct {
	Height float64
	Speed  float64
	Angle  float64
}

type Target struct {
	Distance float64
	Height   float64
	Diameter float64
}

type Game struct {
	Darts  Darts
	Target Target
}

const gravity = 9.8

func (g *Game) Solve() (float64, error) {
	radian := 2.0 * math.Pi * g.Darts.Angle / 360.0
	arrive := g.Darts.Height + g.Target.Distance*math.Tan(radian) - gravity*g.Target.Distance*g.Target.Distance/(2.0*g.Darts.Speed*g.Darts.Speed*math.Cos(radian)*math.Cos(radian))
	diff := math.Abs(arrive - g.Target.Height)
	if diff < g.Target.Diameter/2.0 {
		return diff, nil
	} else {
		return 0, fmt.Errorf("miss")
	}
}
