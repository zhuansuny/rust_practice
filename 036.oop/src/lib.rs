pub struct AveragedCollection {
    list:Vec<i32>,
    average:f64,
}

impl AveragedCollection {
    pub fn new(vec:Vec<i32>,average:f64) -> AveragedCollection {
        AveragedCollection {
            list: vec,
            average: average,
        }
    }

    pub fn add(&mut self, value:i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total:i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// pub use self::Summary as SummaryV2;
pub trait Summary {
    fn summarize(&self) -> String;
    fn print(&self,s:String) {
        println!("just print '{}'",s);
    }
}

impl Summary for AveragedCollection {
    fn summarize(&self) -> String {
        format!("Average is {}", self.average())
    }
}


pub struct NewsArticle {
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        String::from("NewsArticle")
    }
}
    
    


    