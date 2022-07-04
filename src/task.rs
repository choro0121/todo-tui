
use regex::Regex;
use lazy_static::lazy_static;
use chrono::{Local, Date, Datelike, Weekday, NaiveDate, Duration};

pub struct Task {
    // example:
    //   (A) 2022-01-02            due:2022-03-07 @context1 @context2 +project "hoge fuga piyo"
    // x (A) 2022-03-04 2022-01-02 due:2022-03-07 @context1 @context2 +project "hoge fuga piyo"
    pub done: bool,
    pub name: String,
    pub priority: Option<char>,
    pub project: Option<String>,
    pub context: Vec<String>,
    pub create: Option<NaiveDate>,
    pub complete: Option<NaiveDate>,
    pub due: Option<NaiveDate>,
}

fn regex_done(txt: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^x").unwrap();
    }

    match RE.captures(txt) {
        Some(cap) => true,
        None => false,
    }
}

fn regex_name(txt: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?:\s|^)"(.*)""#).unwrap();
    }

    match RE.captures(txt) {
        Some(cap) => cap.get(1).unwrap().as_str().to_string(),
        None => "".to_string(),
    }
}

fn regex_priority(txt: &str) -> Option<char> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\(([A-Z])\)").unwrap();
    }

    match RE.captures(txt) {
        Some(cap) => Some(cap.get(1).unwrap().as_str().chars().nth(0).unwrap()),
        None => None,
    }
}

fn regex_project(txt: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s|^)\+([^\s]+)").unwrap();
    }

    match RE.captures(txt) {
        Some(cap) => Some(cap.get(1).unwrap().as_str().to_string()),
        None => None,
    }
}

fn regex_context(txt: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s|^)@([^\s]+)").unwrap();
    }

    let mut context = vec![];
    for caps in RE.captures_iter(txt) {
        context.push(caps.get(1).unwrap().as_str().to_string());
    }

    context
}

fn regex_date(txt: &str) -> [Option<NaiveDate>; 2] {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s|^)(\d{4}-\d{2}-\d{2})").unwrap();
    }

    let mut date: [Option<NaiveDate>; 2] = [None, None];
    for (i, caps) in RE.captures_iter(txt).enumerate() {
        let date_str = caps.get(1).unwrap().as_str();
        date[i] = Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap());

        if i == 1 {
            break;
        }
    }

    date
}

fn regex_due(txt: &str) -> Option<NaiveDate> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s|^)due:(\d{4}-\d{2}-\d{2})").unwrap();
    }

    match RE.captures(txt) {
        Some(cap) => {
            let date_str = cap.get(1).unwrap().as_str();
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap())
        },
        None => None,
    }
}

fn regex_due_keyword(txt: &str) -> Option<NaiveDate> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s|^)due:(yesterday|today|tomorrow|weekend|\d{4}-\d{2}-\d{2})").unwrap();
    }

    match RE.captures(txt) {
        Some(cap) => {
            let string = cap.get(1).unwrap().as_str();
            let today = Local::today();

            match string {
                "yesterday" => Some(today.pred().naive_local()),
                "today" => Some(today.naive_local()),
                "tomorrow" =>  Some(today.succ().naive_local()),
                "weekend" => {
                    let weekday = today.weekday().number_from_monday() as i8;
                    let friday = Weekday::Fri.number_from_monday() as i8;

                    Some(
                        if weekday <= friday {
                            today + Duration::days((friday - weekday).into())
                        }
                        else {
                            today + Duration::days((7 - weekday + friday).into())
                        }
                        .naive_local()
                    )
                },
                _ => Some(NaiveDate::parse_from_str(string, "%Y-%m-%d").unwrap()),
            }
        },
        None => None,
    }
}

fn parse_date(done: bool, date: [Option<NaiveDate>; 2]) -> (Option<NaiveDate>, Option<NaiveDate>) {
    if done {
        (date[1], date[0])
    }
    else {
        (date[0], None)
    }
}


impl Task {
    pub fn from_string(txt: &str) -> Self {
        let done = regex_done(txt);
        let name = regex_name(txt);
        let priority = regex_priority(txt);
        let project = regex_project(txt);
        let context = regex_context(txt);
        let date = regex_date(txt);
        let due = regex_due(txt);
        let (create, complete) = parse_date(done, date);

        Self {
            done,
            name,
            priority,
            project,
            context,
            create,
            complete,
            due,
        }
    }

    pub fn new(txt: &str) -> Self {
        let name = regex_name(txt);
        let priority = regex_priority(txt);
        let project = regex_project(txt);
        let context = regex_context(txt);
        let due = regex_due_keyword(txt);

        Self {
            done: false,
            name,
            priority,
            project,
            context,
            create: None,
            complete: Some(Local::today().naive_local()),
            due,
        }
    }

    pub fn complete(&mut self) {
        self.done = true;
        self.complete = Some(Local::today().naive_local());
    }

    pub fn incomplete(&mut self) {
        self.done = false;
        self.complete = None;
    }
}
