use std::{cmp::Ordering, collections::{HashMap, VecDeque}};

use regex::Regex;
use once_cell::sync::Lazy;

// https://adventofcode.com/2023/day/19

static WORKFLOW_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<name>[a-z]+)\{(?<rules>.+)\}$").unwrap());
static RULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:(?<category>[a-z]+)(?<ordering>[<>=])(?<rating>[0-9]+):)?(?<next>[a-z|A|R]+)").unwrap());

type WorkflowName = String;
type WorkflowRules = Vec<Rule>;
type Workflows = HashMap<WorkflowName, WorkflowRules>;
type Category = String;
type Rating = u64;
type Part = HashMap<Category, Rating>;
type RatingRanges = HashMap<Category, (u64, u64)>;

#[derive(Debug)]
struct Rule {
    category: Category,
    ordering: Ordering,
    rating: Rating,
    next_workflow: WorkflowName,
}

#[derive(Debug)]
struct State {
    workflow: WorkflowName,
    ranges: RatingRanges,
}

impl State {
    fn init() -> Self {
        Self {
            workflow: String::from("in"),
            ranges: HashMap::from([
                (String::from("x"), (1, 4000)),
                (String::from("m"), (1, 4000)),
                (String::from("a"), (1, 4000)),
                (String::from("s"), (1, 4000)),
            ]),
        } 
    }
}

fn parse_workflow(workflow: &str, workflows: &mut Workflows) {
    let workflow_captures = WORKFLOW_RE.captures(workflow).unwrap();

    let workflow_name = &workflow_captures["name"];
    let rules = &workflow_captures["rules"];

    rules.split(',').for_each(|r| {
        let rule_captures = RULE_RE.captures(r).unwrap();

        let category = {
            match rule_captures.name("category") {
                Some(s) => s.as_str().to_owned(),
                None => "".to_owned(),
            }
        };
        let ordering = {
            let symbol = rule_captures.name("ordering");
            match symbol {
                Some(s) => {
                    match s.as_str() {
                        "<" => Ordering::Less,
                        ">" => Ordering::Greater,
                        "=" => Ordering::Equal,
                        _ => panic!("Unrecognized ordering symbol {}", s.as_str()),
                    }
                }
                None => Ordering::Equal,
            }
        };
        let rating = {
            match rule_captures.name("rating") {
                Some(s) => s.as_str().parse::<Rating>().unwrap(),
                None => 0,
            }
        };
        let next_workflow = rule_captures["next"].to_owned();

        let rule = Rule {
            category,
            ordering,
            rating,
            next_workflow,
        };

        workflows.entry(workflow_name.to_owned()).or_insert_with(Vec::new).push(rule);

    });
}

fn parse_part(part: &str) -> Part {
    let part_ratings = part.trim_start_matches('{').trim_end_matches('}');
    let mut part: Part = HashMap::new();

    for x in part_ratings.split(',') {
        let (category, rating) = {
            let mut x = x.split('=');
            (x.next().unwrap().to_owned(), x.next().unwrap().parse::<u64>().unwrap())
        };

        part.insert(category, rating);
    }

    part
}

fn check_rule(part: &Part, rule: &Rule) -> bool {
    if rule.category == "" {
        return true;
    }

    let part_rating = part.get(&rule.category).unwrap();

    part_rating.cmp(&rule.rating) == rule.ordering
}

fn check_part(part: &Part, workflows: &Workflows) -> bool {
    let mut workflow_name = "in";

    loop {
        if workflow_name == "A" {
            return true;
        }

        if workflow_name == "R" {
            return false;
        }

        let rules = workflows.get(workflow_name).unwrap();

        for rule in rules {
            if check_rule(part, &rule) {
                workflow_name = &rule.next_workflow;
                break;
            }
        }
    }
}

fn count_accepted_rating_combinations(workflows: &Workflows) -> u64 {
    let mut combinations = 0;
    let mut queue = VecDeque::from([State::init()]);
    
    while let Some(State { workflow, mut ranges }) = queue.pop_front() {
        if workflow == "A" {
            combinations += ranges.values().map(|r| r.1 - r.0 + 1).product::<u64>();
            continue;
        }

        if workflow == "R" {
            continue;
        } 

        let rules = workflows.get(&workflow).unwrap();
        for Rule { category, ordering, rating, next_workflow } in rules {
            if category == "" {
                queue.push_back(State {
                    workflow: next_workflow.clone(),
                    ranges: ranges.clone(),
                })
            } else {
                let mut _ranges = ranges.clone();
                let range = _ranges.get(category).unwrap();
                let _range = match ordering {
                    Ordering::Less if range.0 < *rating => {
                        if range.1 >= *rating {
                            ranges.insert(category.clone(), (*rating, range.1));
                        }

                        (range.0, range.1.min(*rating - 1))
                    }
                    Ordering::Greater if range.1 > *rating => {
                        if range.0 <= *rating {
                            ranges.insert(category.clone(), (range.0, *rating));
                        }

                        (range.0.max(*rating + 1), range.1)
                    }
                    _ => (0, 0),
                };

                if _range != (0, 0) {
                    _ranges.insert(category.clone(), _range);
                    queue.push_back(State {
                        workflow: next_workflow.clone(),
                        ranges: _ranges,
                    })
                }
            }
        }
    }

    combinations
}

fn solve_part_1(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    
    let mut workflows: Workflows = HashMap::new();
    input.next().unwrap().lines().for_each(|workflow| parse_workflow(workflow, &mut workflows));

    let parts = input.next().unwrap().lines().map(parse_part);

    parts.filter(|part| check_part(part, &workflows)).map(|part| part.values().sum::<Rating>()).sum::<u64>()
}

fn solve_part_2(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    
    let mut workflows: Workflows = HashMap::new();
    input.next().unwrap().lines().for_each(|workflow| parse_workflow(workflow, &mut workflows));

    count_accepted_rating_combinations(&workflows)
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
