package ticketcounter

type Problem struct {
	customers    uint
	processTimes []uint
}

func NewProblem(customers uint, processTimes []uint) *Problem {
	return &Problem{customers, processTimes}
}

type counter struct {
	empty     bool
	processed uint
}

func (p *Problem) Solve() uint {
	numOfCounters := len(p.processTimes)
	counters := []counter{}
	for i := 0; i < numOfCounters; i++ {
		counters = append(counters, counter{empty: true, processed: 0})
	}

	var time uint = 0
	customers := p.customers

	for {
		for i := 0; i < numOfCounters; i++ {
			if customers > 0 && counters[i].empty {
				counters[i].empty = false
				counters[i].processed = 0
				customers -= 1
			}
		}
		if customers == 0 && p.allEmpty(counters) {
			break
		}
		for i := 0; i < numOfCounters; i++ {
			if !counters[i].empty {
				counters[i].processed += 1
			}
		}
		for i := 0; i < numOfCounters; i++ {
			if !counters[i].empty {
				if counters[i].processed >= p.processTimes[i] {
					counters[i].empty = true
					counters[i].processed = 0
				}
			}
		}
		time += 1
	}

	return time
}

func (p *Problem) allEmpty(counters []counter) bool {
	for _, counter := range counters {
		if !counter.empty {
			return false
		}
	}
	return true
}

/*
       let mut counters = Vec::new();
       for _p in &process_times {
           counters.push(None);
       }
       Problem{customers, process_times, counters}
   }

   fn solve(&mut self) -> u32 {
       let mut time = 0;
       let mut customers = self.customers;
       loop {
           for i in 0..self.counters.len(){
               if customers > 0 && self.counters[i] == None {
                   self.counters[i] = Some(0);
                   customers -= 1;
               }
           }
           if customers == 0 && self.all_empty(){
               break;
           }
           for i in 0..self.counters.len(){
               if let Some(n) = self.counters[i]{
                   self.counters[i] = Some(n+1);
               }
           }
           for i in 0..self.counters.len(){
               if let Some(n) = self.counters[i]{
                   if n >= self.process_times[i]{
                       self.counters[i] = None;
                   }
               }
           }
           time += 1;
       }
       time

*/
