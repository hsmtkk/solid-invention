struct Request {
    number: usize,
    action: String,
}

impl Request {
    fn new(number:usize, action:&str) -> Request {
        Request{number, action:action.to_string()}
    }
}

#[derive(Debug, PartialEq)]
struct Response {
    number: Option<usize>,
    actions: Option<Vec<String>>,
}

impl Response {
    fn new_number(number:usize) -> Response {
        Response{number:Some(number), actions:None}
    }

    fn new_actions(actions:Vec<&str>) -> Response {
        let actions: Vec<String> = actions.iter().map(|x| x.to_string()).collect();
        Response{number:None, actions:Some(actions)}
    }
}

fn process(end:usize, requests:&[Request]) -> Vec<Response> {
    let mut resps: Vec<Response> = Vec::new();
    for clock in 1..end+1 {
        let mut actions: Vec<&str> = Vec::new();
        for req in requests {
            if clock % req.number == 0 {
                actions.push(&req.action);
            }
        }
        if actions.len()==0 {
            resps.push(Response::new_number(clock));
        } else {
            resps.push(Response::new_actions(actions));
        }
    }
    resps
}

#[cfg(test)]
mod  tests {
    use super::{Request, Response};

    #[test]
    fn test0(){
        let requests = vec![
            Request::new(2,"foo"),
            Request::new(3, "bar"),
        ];
        let want = vec![
            Response::new_number(1),
            Response::new_actions(vec!["foo"]),
            Response::new_actions(vec!["bar"]),
            Response::new_actions(vec!["foo"]),
            Response::new_number(5),
            Response::new_actions(vec!["foo","bar"]),
        ];
        let got = super::process(6, &requests);
        assert_eq!(want.len(),got.len());
        for i in 0..want.len(){
            assert_eq!(want[i], got[i]);
        }
    }
}