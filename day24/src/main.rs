use std::{collections::HashMap, fs};

struct Gate<'a> {
    x: &'a str,
    y: &'a str,
    z: &'a str,
    op: &'a str,
    x_val: Option<&'a str>,
    y_val: Option<&'a str>,
    z_val: Option<&'a str>,
}

const TRUE: Option<&str> = Some("1");
const FALSE: Option<&str> = Some("0");
const AND: &str = "AND";
const OR: &str = "OR";
const XOR: &str = "XOR";

impl Gate<'_> {
    fn solve_for_x(&mut self) {
        if self.x_val.is_some() || self.y_val.is_none() || self.z_val.is_none() {
            return;
        }
        self.x_val = match self.op {
            AND => {
                if self.z_val == TRUE {
                    TRUE
                } else if self.y_val == TRUE {
                    FALSE
                } else {
                    None
                }
            }
            OR => {
                if self.z_val == TRUE && self.y_val == FALSE {
                    TRUE
                } else {
                    None
                }
            }
            XOR => {
                if self.z_val == TRUE {
                    if self.y_val == TRUE {
                        FALSE
                    } else {
                        TRUE
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
    }

    fn solve_for_y(&mut self) {
        if self.x_val.is_none() || self.y_val.is_some() || self.z_val.is_none() {
            return;
        }
        self.y_val = match self.op {
            AND => {
                if self.z_val == TRUE {
                    TRUE
                } else if self.x_val == TRUE {
                    FALSE
                } else {
                    None
                }
            }
            OR => {
                if self.z_val == TRUE && self.x_val == FALSE {
                    TRUE
                } else {
                    None
                }
            }
            XOR => {
                if self.z_val == TRUE {
                    if self.x_val == TRUE {
                        FALSE
                    } else {
                        TRUE
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
    }

    fn solve_for_z(&mut self) {
        if self.z_val.is_some() || self.x_val.is_none() || self.y_val.is_none() {
            return;
        }
        self.z_val = match self.op {
            AND => {
                if self.y_val == TRUE && self.x_val == TRUE {
                    TRUE
                } else {
                    FALSE
                }
            }
            OR => {
                if self.y_val == TRUE || self.x_val == TRUE {
                    TRUE
                } else {
                    FALSE
                }
            }
            XOR => {
                if self.y_val != self.x_val {
                    TRUE
                } else {
                    FALSE
                }
            }
            _ => None,
        };
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut parts = contents.split("\n\n");
    let mut wires: HashMap<&str, &str> = HashMap::new();
    for line in parts.next().unwrap().lines() {
        let mut segments = line.split(": ");
        let wire = segments.next().unwrap();
        let value = segments.next().unwrap();
        wires.insert(wire, value);
    }

    let mut gates: Vec<Gate<'_>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            Gate {
                x: parts[0],
                op: parts[1],
                y: parts[2],
                z: parts[4],
                x_val: None,
                y_val: None,
                z_val: None,
            }
        })
        .collect();

    loop {
        let mut mutated = false;
        for gate in gates.iter_mut() {
            if gate.x_val.is_none() {
                if let Some(value) = wires.get(gate.x) {
                    gate.x_val = Some(value);
                } else {
                    gate.solve_for_x();
                }
                mutated = true;
            } else {
                wires.insert(gate.x, gate.x_val.unwrap());
            }

            if gate.y_val.is_none() {
                if let Some(value) = wires.get(gate.y) {
                    gate.y_val = Some(value);
                } else {
                    gate.solve_for_y();
                }
                mutated = true;
            } else {
                wires.insert(gate.y, gate.y_val.unwrap());
            }

            if gate.z_val.is_none() {
                if let Some(value) = wires.get(gate.z) {
                    gate.z_val = Some(value);
                } else {
                    gate.solve_for_z();
                }
                mutated = true;
            } else {
                wires.insert(gate.z, gate.z_val.unwrap());
            }
        }

        if !mutated {
            break;
        }
    }

    let mut z_wires: Vec<Gate> = gates
        .into_iter()
        .filter(|gate| gate.z.starts_with("z"))
        .collect();
    z_wires.sort_unstable_by(|a, b| b.z.cmp(a.z));

    let binary: String = z_wires.iter().map(|gate| gate.z_val.unwrap()).collect();
    let part_1 = isize::from_str_radix(&binary, 2).unwrap();
    println!("Part 1: {}", part_1);
}
