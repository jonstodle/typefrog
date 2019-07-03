use crate::typegen::schema_types::{Type, TypeKind};
use crate::typegen::languages::TypeSection;

pub struct Rust;

impl Rust {
    pub fn generate_types(types: &Vec<Type>) -> String {
        let output = types.iter()
            .flat_map(|t| {
                let mut sections = Rust::parse_type(t);
                if sections.len() > 0 {
                    sections.extend(vec![TypeSection::Line(String::from(""))]);
                }
                sections
            })
            .collect();

        super::print(output, 4, 0)
    }

    fn parse_type(t: &Type) -> Vec<TypeSection> {
        match t.kind {
            TypeKind::Object => Rust::generate_object(t),
            TypeKind::Interface => Rust::generate_interface(t),
            TypeKind::Enum => Rust::generate_enum(t),
            TypeKind::Scalar => Rust::generate_scalar(t),
            TypeKind::InputObject => Rust::generate_input_object(t),
            TypeKind::NonNull => Rust::parse_type(&t.of_type.as_ref().as_ref().unwrap()),
            TypeKind::List => Rust::parse_type(&t.of_type.as_ref().as_ref().unwrap()),
            TypeKind::Union => Rust::generate_enum(t),
        }
    }

    fn generate_object(t: &Type) -> Vec<TypeSection> {
        assert_eq!(t.kind, TypeKind::Object);

        vec![
            TypeSection::Line(format!("pub struct {} {{", t.name.as_ref().unwrap())),
            TypeSection::Indent(t.fields.as_ref().unwrap().iter()
                .flat_map(|field| {
                    let name = field.name.as_ref().unwrap();
                    vec![
                        TypeSection::Line(format!(
                            r#"#[serde(rename = "{}")"#,
                            name
                        )),
                        TypeSection::Line(format!(
                            "pub {}: {},",
                            Rust::localize_name(name),
                            Rust::get_type_name(&field.schema_type)
                        ))
                    ]
                }).collect()),
            TypeSection::Line("}".into()),
        ]
    }

    fn generate_interface(t: &Type) -> Vec<TypeSection> {
        assert_eq!(t.kind, TypeKind::Interface);

        vec![
            TypeSection::Line(format!("pub enum {} {{", t.name.as_ref().unwrap())),
            TypeSection::Indent(t.possible_types.as_ref().unwrap().iter()
                .map(|t| {
                    TypeSection::Line(format!(
                        "{0}({0}),",
                    t.name.as_ref().unwrap()))
                }).collect()),
            TypeSection::Line("}".into())
        ]
    }

    fn generate_enum(t: &Type) -> Vec<TypeSection> {
        assert_eq!(t.kind, TypeKind::Enum);

        vec![
            TypeSection::Line(format!("pub enum {} {{", t.name.as_ref().unwrap())),
            TypeSection::Indent(t.enum_values.as_ref().unwrap().iter()
                .map(|ev| {
                    TypeSection::Line(format!(
                        "{0},",
                        &ev.name))
                }).collect()),
            TypeSection::Line("}".into())
        ]
    }

    fn generate_scalar(t: &Type) -> Vec<TypeSection> {
        assert_eq!(t.kind, TypeKind::Scalar);

        match &t.name.as_ref().unwrap()[..] {
            "Int" | "Float" | "String" | "Boolean" | "ID" => vec![],
            _ => {
                vec![
                    TypeSection::Line(format!("pub struct {}(String);", t.name.as_ref().unwrap())),
                ]
            }
        }
    }

    fn generate_input_object(t: &Type) -> Vec<TypeSection> {
        assert_eq!(t.kind, TypeKind::InputObject);

        vec![
            TypeSection::Line(format!("pub struct {} {{", t.name.as_ref().unwrap())),
            TypeSection::Indent(t.input_fields.as_ref().unwrap().iter()
                .map(|field| {
                    TypeSection::Line(format!(
                        "pub {}: {},",
                        Rust::localize_name(&field.name),
                        Rust::get_type_name_without_option(&field.schema_type)))
                }).collect()),
            TypeSection::Line("}".into()),
        ]
    }

    fn get_type_name_without_option(t: &Type) -> String {
        let name = Rust::get_type_name(t);

        if t.kind.eq(&TypeKind::NonNull) && name.starts_with("Option<") {
            String::from(&name[7..(name.len() - 1)])
        } else {
            name
        }
    }

    fn get_type_name(t: &Type) -> String {
        match t.kind {
            TypeKind::List => {
                let mut name = format!("Option<Vec<{}>>", Rust::get_type_name(&t.of_type.as_ref().as_ref().unwrap()));

                if t.of_type.as_ref().as_ref().unwrap().kind.eq(&TypeKind::NonNull) &&
                    name.starts_with("Option<Vec<Option<") {
                    name.replace_range(11..18, "");
                    name.remove(name.len() - 1);
                }

                name
            },
            TypeKind::NonNull => {
               Rust::get_type_name(&t.of_type.as_ref().as_ref().unwrap())
            },
            _ => {
                let name = if let Some(ref n) = t.name {
                    let n = &n[..];
                    match n {
                        "Int" => String::from("isize"),
                        "Float" => String::from("f64"),
                        "Boolean" => String::from("bool"),
                        "ID" => String::from("String"),
                        _ => String::from(n),
                    }
                } else {
                    String::from("")
                };
                format!("Option<{}>", &name)
            }
        }
    }

    fn localize_name(name: &str) -> String {
        name.char_indices()
            .flat_map(|(idx, c)|
                if c.is_uppercase() {
                    let mut v = vec![];
                    if idx != 0 {
                        v.push('_');
                    }
                    v.extend(c.to_lowercase().into_iter());
                    v
                } else {
                    vec![c]
                })
            .collect::<String>()
    }
}

