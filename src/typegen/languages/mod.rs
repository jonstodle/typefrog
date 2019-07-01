pub use rust::Rust;

mod rust;

pub enum TypeSection {
    Line(String),
    Indent(Vec<TypeSection>),
}

fn print(sections: Vec<TypeSection>, indent_size: usize, current_indent: usize) -> String {
    let mut output = String::new();

    for s in sections {
        match s {
            TypeSection::Line(line) => {

                output.push_str(&" ".repeat(current_indent));
                output.push_str(&line);
                output.push_str("\n");
            }
            TypeSection::Indent(sections) => output.push_str(&print(sections, indent_size, current_indent + indent_size)),
        };
    };

    output
}
