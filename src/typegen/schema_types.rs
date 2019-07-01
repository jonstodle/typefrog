use serde::Deserialize;

#[derive(Deserialize)]
pub struct Schema {
    pub types: Vec<Type>,
    #[serde(rename = "queryType")]
    pub query_type: TypeReference,
    #[serde(rename = "mutationType")]
    pub mutation_type: Option<TypeReference>,
    #[serde(rename = "subscriptionType")]
    pub subscription_type: Option<TypeReference>,
    pub directives: Vec<Directive>,
}

#[derive(Deserialize)]
pub struct TypeReference {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub name: Option<String>,
    pub description: Option<String>,

    // Object and Interface only
    pub fields: Option<Vec<Field>>,

    // Object only
    pub interfaces: Option<Vec<Type>>,

    // Interface and Union only
    #[serde(rename = "possibleTypes")]
    pub possible_types: Option<Vec<Type>>,

    // Enum only
    #[serde(rename = "enumValues")]
    pub enum_values: Option<Vec<EnumValue>>,

    // InputObject only
    #[serde(rename = "inputFields")]
    pub input_fields: Option<Vec<InputValue>>,

    // NonNull and List only
    #[serde(rename = "ofType")]
    pub of_type: Box<Option<Type>>,
}

#[derive(Deserialize)]
pub struct Field {
    pub name: Option<String>,
    pub description: Option<String>,
    pub args: Vec<InputValue>,
    #[serde(rename = "type")]
    pub schema_type: Type,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
}

#[derive(Deserialize)]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub schema_type: Type,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
}

#[derive(Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum TypeKind {
    #[serde(alias = "SCALAR")]
    Scalar,
    #[serde(alias = "OBJECT")]
    Object,
    #[serde(alias = "INTERFACE")]
    Interface,
    #[serde(alias = "UNION")]
    Union,
    #[serde(alias = "ENUM")]
    Enum,
    #[serde(alias = "INPUT_OBJECT")]
    InputObject,
    #[serde(alias = "LIST")]
    List,
    #[serde(alias = "NON_NULL")]
    NonNull,
}

#[derive(Deserialize)]
pub struct Directive {
    pub name: String,
    pub description: Option<String>,
    pub locations: Vec<DirectiveLocation>,
    pub args: Vec<InputValue>,
}

#[derive(Deserialize)]
pub enum DirectiveLocation {
    #[serde(rename = "QUERY")]
    Query,
    #[serde(rename = "MUTATION")]
    Mutation,
    #[serde(rename = "SUBSCRIPTION")]
    Subscription,
    #[serde(rename = "FIELD")]
    Field,
    #[serde(rename = "FRAGMENT_DEFINITION")]
    FragmentDefinition,
    #[serde(rename = "FRAGMENT_SPREAD")]
    FragmentSpread,
    #[serde(rename = "INLINE_FRAGMENT")]
    InlineFragment,
    #[serde(rename = "SCHEMA")]
    Schema,
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "FIELD_DEFINITION")]
    FieldDefinition,
    #[serde(rename = "ARGUMENT_DEFINITION")]
    ArgumentDefinition,
    #[serde(rename = "INTERFACE")]
    Interface,
    #[serde(rename = "UNION")]
    Union,
    #[serde(rename = "ENUM")]
    Enum,
    #[serde(rename = "ENUM_VALUE")]
    EnumValue,
    #[serde(rename = "INPUT_OBJECT")]
    InputObject,
    #[serde(rename = "INPUT_FIELD_DEFINITION")]
    InputFieldDefinition,
}
