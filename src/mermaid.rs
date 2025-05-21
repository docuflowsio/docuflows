pub fn generate_mermaid_diagram(steps: &[String]) -> String {
    let mut diagram = String::from("graph TD\n");
    for pair in steps.windows(2) {
        diagram.push_str(&format!("    {} --> {}\n", pair[0], pair[1]));
    }
    diagram
}