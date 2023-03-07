use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::fs;
use std::process::exit;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "machine.pest"]
struct TuringMachineParser;

#[derive(Debug, Clone, Copy)]
enum Movement {
    Left,
    Right,
    Reject,
    Accept,
    Output,
}

#[derive(Debug, Clone)]
struct Destination {
    write: String,
    result: Movement,
    next: Option<String>,
}
type TMRule = HashMap<String, Destination>;
type TransitionRule = HashMap<String, TMRule>;

#[derive(Debug, Clone)]
struct RuleDecl {
    state: String,
    transitions: Vec<TransitionDecl>,
}

#[derive(Debug, Clone)]
struct TransitionDecl {
    read: String,
    write: String,
    action: Movement,
    next: Option<String>,
}
impl TransitionDecl {
    fn into(self) -> (String, Destination) {
        (
            self.read,
            Destination {
                write: self.write,
                result: self.action,
                next: self.next,
            },
        )
    }
}

#[allow(clippy::too_many_lines)]
fn run_file(
    filename: &str,
    input: &[&str],
    default: &str,
) -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string(filename)?;
    let rules =
        TuringMachineParser::parse(Rule::Machine, &program)?.collect::<Vec<_>>();
    let rules = rules
        .into_iter()
        .map(|rule| {
            let mut rule_parts = rule.into_inner();
            RuleDecl {
                state: rule_parts
                    .next()
                    .expect("Always exists by grammar definition")
                    .as_str()
                    .to_string(),
                transitions: rule_parts
                    .map(|transition| {
                        let mut parts = transition.into_inner();
                        TransitionDecl {
                            read: parts
                                .next()
                                .expect("Always exists by grammar definition")
                                .as_str()
                                .to_string(),
                            write: parts
                                .next()
                                .expect("Always exists by grammar definition")
                                .as_str()
                                .to_string(),
                            action: match parts
                                .next()
                                .expect("Always exists by grammar definition")
                                .as_str()
                            {
                                "<" => Movement::Left,
                                ">" => Movement::Right,
                                "=" => Movement::Accept,
                                "!" => Movement::Reject,
                                "?" => Movement::Output,
                                _ => {
                                    unreachable!(
                                        "Other inputs would not be accepted by the \
                                         grammar"
                                    )
                                },
                            },
                            next: parts.next().map(|str| str.as_str().to_string()),
                        }
                    })
                    .collect(),
            }
        })
        .collect::<Vec<_>>();
    let initial_state = rules[0].state.clone();
    let mut state = &*initial_state;
    let rules = rules
        .into_iter()
        .map(|rule| {
            (
                rule.state,
                rule.transitions
                    .into_iter()
                    .map(TransitionDecl::into)
                    .collect(),
            )
        })
        .collect::<TransitionRule>();
    let mut tape = (0isize..)
        .zip(input.iter().copied())
        .collect::<HashMap<_, _>>();
    let mut head = 0;
    loop {
        let read_cell = tape.get(&head).copied().unwrap_or(default);
        let transitions = rules.get(state).unwrap_or_else(|| {
            eprintln!("No rules found for state {state:?}. Aborting.");
            let (head_index, arranged_tape) = arrange_tape(&tape, default, head);
            eprintln!("Tape state at error: {arranged_tape:?}");
            eprintln!(
                "                      {}{}",
                " ".repeat(
                    arranged_tape[..head_index]
                        .iter()
                        .map(|val| val.len() + 4)
                        .sum::<usize>()
                ),
                "^".repeat(arranged_tape[head_index].len() + 2)
            );
            exit(1);
        });
        let dest = transitions.get(read_cell).unwrap_or_else(|| {
            eprintln!(
                "No rules found for reading {read_cell:?} at state {state:?}. \
                 Aborting."
            );
            let (head_index, arranged_tape) = arrange_tape(&tape, default, head);
            eprintln!("Tape state at error: {arranged_tape:?}");
            eprintln!(
                "                      {}{}",
                " ".repeat(
                    arranged_tape[..head_index]
                        .iter()
                        .map(|val| val.len() + 4)
                        .sum::<usize>()
                ),
                "^".repeat(arranged_tape[head_index].len() + 2)
            );
            exit(1);
        });
        tape.insert(head, &dest.write);
        match dest.result {
            Movement::Left => head -= 1,
            Movement::Right => head += 1,
            Movement::Reject => {
                println!("The turing machine has rejected the input.");
                break;
            },
            Movement::Accept => {
                println!("The turing machine has accepted the input.");
                break;
            },
            Movement::Output => {
                println!("The turing machine returns this result:");
                let (_, tape) = arrange_tape(&tape, default, head);
                println!("{tape:?}");
                break;
            },
        }
        if let Some(next) = dest.next.as_deref() {
            state = next;
        }
    }

    Ok(())
}

#[allow(clippy::cast_sign_loss)]
fn arrange_tape<'a>(
    tape: &HashMap<isize, &'a str>,
    trim: &str,
    head: isize,
) -> (usize, Vec<&'a str>) {
    let mut values = tape.iter().map(|(&i, &s)| (i, s)).collect::<Vec<_>>();
    values.sort_by_key(|&(i, _)| i);
    let mut found_first = false;
    let mut end = 0;
    values.retain(|&val| {
        if val.1 != trim {
            found_first = true;
            end = val.0;
        }
        found_first
    });
    end -= values[0].0;
    values.truncate(end as usize + 1);
    (
        (head - values[0].0) as usize,
        values.into_iter().map(|(_, s)| s).collect::<Vec<_>>(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    let progname = args.next().expect("Should be the binary name");
    let filename = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {progname} <input file> <input> [default = \"#\"]");
        exit(1);
    });
    let raw_input = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {progname} <input file> <input> [default = \"#\"]");
        exit(1);
    });
    let input = raw_input
        .as_str()
        .split("")
        .filter(|val| !val.is_empty())
        .collect::<Vec<_>>();
    let default_owned = args.next();
    let default = default_owned.as_deref().unwrap_or("#");
    run_file(&filename, &input, default)
}
