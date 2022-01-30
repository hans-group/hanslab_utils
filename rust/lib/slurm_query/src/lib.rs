use regex::Regex;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Job {
    pub id: String,
    pub jobname: String,
    pub username: String,
    pub state: String,
    pub partition: String,
    pub numnodes: String,
    pub numtasks: String,
    pub runtime: String,
    pub workdir: String,
}

impl FromStr for Job {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Job::extract_value(s, r"JobId=\d{6}");
        let jobname = Job::extract_value(s, r"JobName=[\[\]\w\-_\d.]+");
        let username = Job::extract_value(s, r"UserId=\w+");
        let state = Job::extract_value(s, r"JobState=\w+");
        let partition = Job::extract_value(s, r"Partition=\w+");
        let numnodes = Job::extract_value(s, r"NumNodes=\d+");
        let numtasks = Job::extract_value(s, r"NumCPUs=\d+");
        let runtime = Job::extract_value(s, r"RunTime=[\-\d]+:\d+:\d+");
        let workdir = Job::extract_value(s, r"WorkDir=[/\w\-_\d.]+");

        Ok(Job {
            id,
            jobname,
            username,
            state,
            partition,
            numnodes,
            numtasks,
            runtime,
            workdir,
        })
    }
}

impl Job {
    fn extract_value(s: &str, re_str: &str) -> String {
        let re = Regex::new(re_str).unwrap();
        let caps = re.captures(s).unwrap();
        let val = &caps[0].split('=').last();
        match val {
            Some(s) => s.to_string(),
            None => String::from("None"),
        }
    }
}
