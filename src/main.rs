use make::{MakeManager, Makefile, MakeTarget, TargetType};
use std::collections::BTreeMap;

fn main() {
    let mut mgr = MakeManager::new("make");
    
    let mut makefile = Makefile {
        id: "Makefile".to_string(),
        targets: BTreeMap::new(),
        variables: BTreeMap::new(),
        default_target: Some("all".to_string()),
    };
    
    let target = MakeTarget {
        name: "all".to_string(),
        target_type: TargetType::Phony,
        dependencies: vec!["program".to_string()],
        commands: vec!["echo Build complete".to_string()],
        phony: true,
    };
    
    makefile.targets.insert(target.name.clone(), target);
    
    if let Err(e) = mgr.add_makefile(makefile) {
        eprintln!("Failed to add makefile: {}", e);
        return;
    }
    
    println!("SigmaOS-make operational: {} makefiles", mgr.count());
}
