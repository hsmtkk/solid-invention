struct Route {
    rain: Vec<u32>,
}

impl Route {
    fn new(rain:Vec<u32>) -> Route {
        Route{rain}
    }
}

struct Problem {
    limit: u32,
    routes: Vec<Route>,
}

impl Problem {
    fn new(limit:u32, routes:Vec<Route>) -> Problem {
        Problem{limit, routes}
    }

    fn solve(&self) -> Option<Vec<usize>>{
        let mut routes = Vec::new();
        for (i, route) in self.routes.iter().enumerate(){
            if self.can_pass(route) {
                routes.push(i);
            }
        }
        if routes.is_empty() {
            None
        } else {
            Some(routes)
        }
    }

    fn can_pass(&self, route: &Route) -> bool {
        for rain in &route.rain {
            if self.limit <= *rain {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;
    use super::Route;

    #[test]
    fn test0(){
        let routes = vec![Route::new(vec![100,200,20]), Route::new(vec![100,20,20]), Route::new(vec![500,20,20])];
        let problem = Problem::new(200, routes);
        let want = Some(vec![1]);
        let got = problem.solve();
        assert_eq!(want, got);
    }
}