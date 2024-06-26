trait Summable {
    fn sum(&self) ->i32;
}

impl Summable for Vec<i32>{
    fn sum(&self) -> i32 {
        self.iter().sum()
    }
}
fn print_sum<T : Summable>(items: T) {
    println!("Sum is {}", items.sum())
}

fn main() {
    let numbers = vec![1,2,3,4,5];
    print_sum(numbers);
}


trait Multipliable {
    fn product(&self) ->i32;
}

impl Multipliable for Vec<i32>{
    fn product(&self) -> i32 {
        self.iter().product()
    }
}
fn print_product<T : Multipliable>(items: T) {
    println!("product is {}", items.product())
}

fn main() {
    let numbers = vec![1,2,3,4,5];
    print_product(numbers);
}






struct News {
    name: String,
    headlines: String,
    number: i32,
}

trait NewsPaper {
    fn summarize(&self) -> String;
    
    fn create_summary(&self) -> String {
        format!("{}: {}", self.name(), self.headlines())
    }

    fn name(&self) -> &str;
    fn headlines(&self) -> &str;
    fn number(&self) -> i32;
}

impl NewsPaper for News {
    fn summarize(&self) -> String {
        format!("News today {}", self.name())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn headlines(&self) -> &str {
        &self.headlines
    }

    fn number(&self) -> i32 {
        self.number
    }
}

fn earth_for_aliens(items: impl NewsPaper) {
    println!("Hello, these are the news {}", items.create_summary());
}

fn return_summary() -> impl NewsPaper{
 return News{name: String::from("Taifa Leo"), headlines:String::from("Breaking News!"), number: 1};

}

fn main() {
    let news = News{name: String::from("Taifa Leo"), headlines:String::from("Breaking News!"), number: 1};
   
    println!("Summarization: {}", news.summarize());
    println!("Summary: {}", news.create_summary());
    println!("Details:");
    println!("Name: {}", news.name());
    println!("Headlines: {}", news.headlines());
    println!("Number: {}", news.number());

    earth_for_aliens(news);

    let res = return_summary();
    println!("{}", res.summarize())
}