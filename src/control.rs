pub enum OutputControl {
    TimeStep(f64),
    Iteration(usize),
    Final,    
}