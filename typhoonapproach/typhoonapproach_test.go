package typhoonapproach_test

import (
	"testing"

	ta "github.com/hsmtkk/solid-invention/typhoonapproach"
	"github.com/stretchr/testify/assert"
)

/*
3 200
100 200 20
100 20 20
500 20 20
*/
func Test0(t *testing.T) {
	routes := []ta.Route{ta.NewRoute(100, 200, 20), ta.NewRoute(100, 20, 20), ta.NewRoute(500, 20, 20)}
	problem := ta.NewProblem(200, routes)
	gotFound, gotRoutes := ta.Solve(problem)
	wantRoutes := []ta.Route{ta.NewRoute(100, 20, 20)}
	assert.True(t, gotFound)
	assert.Equal(t, wantRoutes, gotRoutes)
}

/*
3 100
1000 1000 1000
1000 1000 1000
1000 1000 1000
*/
func Test1(t *testing.T) {
	routes := []ta.Route{ta.NewRoute(1000, 1000, 1000), ta.NewRoute(1000, 1000, 1000), ta.NewRoute(1000, 1000, 1000)}
	problem := ta.NewProblem(100, routes)
	gotFound, _ := ta.Solve(problem)
	assert.False(t, gotFound)
}

/*
3 50
40 40 40
40 40 40
40 40 40
*/
func Test2(t *testing.T) {
	routes := []ta.Route{ta.NewRoute(40, 40, 40), ta.NewRoute(40, 40, 40), ta.NewRoute(40, 40, 40)}
	problem := ta.NewProblem(50, routes)
	gotFound, gotRoutes := ta.Solve(problem)
	wantRoutes := []ta.Route{ta.NewRoute(40, 40, 40), ta.NewRoute(40, 40, 40), ta.NewRoute(40, 40, 40)}
	assert.True(t, gotFound)
	assert.Equal(t, wantRoutes, gotRoutes)
}
