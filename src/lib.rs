pub struct Scheduler {
    jobs: Vec<Job>,
}

struct Job {
    expression: String,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { jobs: vec![] }
    }
    
    pub fn add_job<F>(&mut self, expression: &str, _handler: F)
    where F: Fn() + Send + 'static {
        self.jobs.push(Job { expression: expression.to_string() });
    }
    
    pub async fn start(&self) {
        println!("Scheduler started with {} jobs", self.jobs.len());
    }
}
