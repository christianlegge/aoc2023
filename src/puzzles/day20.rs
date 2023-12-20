use hashbrown::HashMap;

#[test]
fn test() {
    //     solve(String::from(
    //         "broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a",
    //     ));
    solve(String::from(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    ));
}

#[derive(Clone, Eq, Debug, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    pending: Option<Pulse>,
    targets: Vec<String>,
}

#[derive(Debug)]
struct ModuleArray {
    modules: HashMap<String, Module>,
    lows: usize,
    highs: usize,
    module_map: HashMap<String, Vec<String>>,
}

impl Module {
    fn new(data: &str) -> Self {
        let mut parts = data.split(" -> ");
        let mut name = parts.next().unwrap().to_owned();
        let prefix = name.chars().next().unwrap();
        let module_type = match prefix {
            '%' => ModuleType::FlipFlop(false),
            '&' => ModuleType::Conjunction(HashMap::new()),
            _ => ModuleType::Broadcaster,
        };
        if module_type != ModuleType::Broadcaster {
            name = name[1..].to_owned();
        }
        let targets = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        Module {
            name,
            module_type,
            pending: None,
            targets,
        }
    }

    fn receive_pulse(&mut self, origin: &str, pulse: Pulse) {
        use ModuleType::*;
        match (&mut self.module_type, pulse) {
            (Broadcaster, p) => {
                // self.send_pulses(p);
                self.pending = Some(p);
            }
            (FlipFlop(ref mut state), Pulse::Low) => {
                *state = !*state;
                self.pending = Some(if *state { Pulse::High } else { Pulse::Low });
            }
            (Conjunction(ref mut states), p) => {
                states.insert(origin.to_string(), p);
                self.pending = Some(if states.iter().all(|(_, p)| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                });
                // self.send_pulses(if states.iter().all(|(_, p)| *p == Pulse::High) {
                //     Pulse::Low
                // } else {
                //     Pulse::High
                // });
            }
            _ => {}
        }
    }

    // fn send_pulses(&mut self, mods: &mut ModuleArray) {
    //     for (target, pulse) in self.pending.drain(..) {
    //         mods.modules
    //             .get_mut(&target)
    //             .unwrap()
    //             .receive_pulse(&self.name, pulse);
    //         if pulse == Pulse::High {
    //             mods.highs += 1;
    //         } else {
    //             mods.lows += 1;
    //         }
    //     }
    // }
}

impl ModuleArray {
    fn add(&mut self, module: Module) {
        let module = module;
        self.module_map
            .insert(module.name.clone(), module.targets.clone());
        self.modules.insert(module.name.clone(), module);
    }

    fn push_button(&mut self) {
        // println!("----------------------------");
        self.modules
            .get_mut("broadcaster")
            .unwrap()
            .receive_pulse("button", Pulse::Low);
        self.lows += 1;
        // println!("button -low-> broadcaster");
        while self.modules.iter().any(|(_, m)| m.pending.is_some()) {
            self.tick();
        }
    }

    fn tick(&mut self) {
        for (name, targets) in self.module_map.clone() {
            if let Some(pulse) = self.modules.get(&name).unwrap().pending {
                for target in targets {
                    // println!(
                    //     "{} -{}-> {}",
                    //     name,
                    //     if pulse == Pulse::High { "high" } else { "low" },
                    //     target
                    // );
                    if let Some([sender, receiver]) = self.modules.get_many_mut([&name, &target]) {
                        receiver.receive_pulse(&sender.name, pulse);
                    }
                    if pulse == Pulse::High {
                        self.highs += 1;
                    } else {
                        self.lows += 1;
                    }
                }
                self.modules.get_mut(&name).unwrap().pending = None;
            }
        }
    }

    fn resolve_inputs(&mut self) {
        for (name, module) in &mut self.modules {
            if let ModuleType::Conjunction(ref mut inputs) = module.module_type {
                self.module_map
                    .keys()
                    .filter(|n| self.module_map.get(*n).unwrap().contains(name))
                    .for_each(|n| {
                        inputs.insert(n.to_string(), Pulse::Low);
                    });
            }
        }
    }

    fn _inspect(&self) {
        for module in self.modules.values() {
            dbg!(&module);
        }
    }
}

pub fn solve(data: String) {
    let mut modules = ModuleArray {
        modules: HashMap::new(),
        module_map: HashMap::new(),
        highs: 0,
        lows: 0,
    };
    for line in data.lines() {
        if line == "" {
            continue;
        }
        println!("{}", line);
        let module = Module::new(line);
        modules.add(module);
    }
    modules.resolve_inputs();
    for _ in 0..1000 {
        modules.push_button();
    }
    println!(
        "highs: {} lows: {} prod: {}",
        modules.highs,
        modules.lows,
        modules.highs * modules.lows
    );
}
