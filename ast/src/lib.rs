use tree_sitter::{Parser, Tree};
use tree_sitter_ziust::LANGUAGE;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
    Type(TypeDeclaration),
    Trait(TraitDeclaration),
    Impl(ImplDeclaration),
    Const(ConstDeclaration),
    Fn(FnDeclaration),
    Let(LetDeclaration),
    LetElse(LetElseDeclaration),
    Null(NullStatement),
    Deferrable(DeferrableStatement),
    Defer(DeferStatement),
    Test(TestStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    template_parameters: Vec<TemplateParameter>,
    generic_parameters: Vec<GenericParameter>,
    members: Vec<StructDeclarationMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclarationMember {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    r#type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    tag_type: Type,
    template_parameters: Vec<TemplateParameter>,
    generic_parameters: Vec<GenericParameter>,
    members: Vec<EnumDeclarationMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclarationMember {
    attributes: Vec<Attribute>,
    name: String,
    r#type: Vec<Type>,
    tag_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    template_parameters: Vec<TemplateParameter>,
    generic_parameters: Vec<GenericParameter>,
    r#type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    template_parameters: Vec<TemplateParameter>,
    members: Vec<TraitMemberDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraitMemberDeclaration {
    Const(TraitMemberDeclarationConst),
    Fn(TraitMemberDeclarationFn),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMemberDeclarationConst {
    attributes: Vec<Attribute>,
    name: String,
    r#type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMemberDeclarationFn {
    attributes: Vec<Attribute>,
    name: String,
    template_parameters: Vec<TemplateParameter>,
    generic_parameters: Vec<GenericParameter>,
    parameters: Vec<FnParameter>,
    return_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplDeclaration {
    attributes: Vec<Attribute>,
    template_parameters: Vec<TemplateParameter>,
    generic_parameters: Vec<GenericParameter>,
    trait_type: Option<Type>,
    target_type: Type,
    members: Vec<ImplMemberDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImplMemberDeclaration {
    Const(ConstDeclaration),
    Fn(FnDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    r#type: Type,
    value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclaration {
    attributes: Vec<Attribute>,
    is_public: bool,
    name: String,
    template_parameters: Vec<TemplateParameter>,
    generic_parameters: Vec<GenericParameter>,
    parameters: Vec<FnParameter>,
    return_type: Type,
    body: BlockExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetDeclaration {
    attributes: Vec<Attribute>,
    is_mut: bool,
    name: String,
    r#type: Option<Type>,
    value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetElseDeclaration {
    attributes: Vec<Attribute>,
    pattern: Pattern,
    value: Expression,
    r#else: BlockExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullStatement {
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeferrableStatement {
    Block(BlockStatement),
    Expression(ExpressionStatement),
    Assignment(AssignmentStatement),
    If(IfExpression),
    For(ForStatement),
    While(WhileStatement),
    Loop(LoopExpression),
    Match(MatchExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeferStatement {
    attributes: Vec<Attribute>,
    labels: Vec<String>,
    when: Option<String>,
    statement: DeferrableStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestStatement {
    attributes: Vec<Attribute>,
    name: String,
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    members: Vec<AttributeMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateParameter {
    name: String,
    candidates: Vec<TemplateParameterValueOr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParameter {
    name: String,
    value: GenericArgument,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Never,
    Tuple(TupleType),
    Reference(ReferenceType),
    Pointer(PointerType),
    Array(ArrayType),
    Slice(SliceType),
    Terminal(TerminalType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Simple(SimpleExpression),
    Block(BlockExpression),
    If(IfExpression),
    Loop(LoopExpression),
    Match(MatchExpression),
    Return(ReturnExpression),
    Break(BreakExpression),
    Continue(ContinueExpression),
    And(AndExpression),
    Or(OrExpression),
    As(AsExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnParameter {
    This(FnParameterSelf), // r#Self?
    Normal(FnParameterNormal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpression {
    label: Option<String>,
    statements: Vec<Statement>,
    return_value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Terminal(PatternTerminal),
    Enum(PatternEnum),
    Struct(PatternStruct),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    label: Option<String>,
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    attributes: Vec<Attribute>,
    expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    lhs: Expression,
    rhs: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    is_const: bool,
    condition: IfCondition,
    if_true: BlockExpression,
    else_if: Vec<ElseIf>,
    r#else: Option<BlockExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pattern: Pattern,
    r#in: SimpleExpression,
    body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    condition: SimpleExpression,
    body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpression {
    label: Option<String>,
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpression {
    expression: SimpleExpression,
    label: Option<String>,
    arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeMember {
    name: ConstReference,
    argument: Option<Group>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateParameterValueOr {
    items: Vec<TemplateParameterValueAnd>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericArgument {
    Terminal(GenericArgumentTerminal),
    Tuple(GenericArgumentTuple),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleType {
    members: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceType {
    is_optional: bool,
    is_mut: bool,
    r#type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointerType {
    is_optional: bool,
    is_mut: bool,
    r#type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    length: Option<Expression>,
    is_mut: bool,
    r#type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliceType {
    is_mut: bool,
    r#type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerminalType {
    name: ConstReference,
    template_arguments: Vec<TemplateArgument>,
    generic_arguments: Vec<GenericArgument>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExpression {
    Literal(Literal),
    Parenthesised(ParenthesisedExpression),
    BuiltinFunctionCall(BuiltinFunctionCallExpression),
    MacroCall(MacroCallExpression),
    Member(MemberExpression),
    ConstReference(ConstReference),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpression {
    label: Option<String>,
    expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakExpression {
    label: Option<String>,
    expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueExpression {
    label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AndExpression {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrExpression {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsExpression {
    expression: Box<Expression>,
    r#type: Box<Type>,
}

// TODO:

pub fn ast(source_code: &str) -> Module {
    let mut parser = Parser::new();
    parser
        .set_language(&LANGUAGE.into())
        .expect("Error loading Ziust grammar");

    let tree: Tree = parser.parse(source_code, None).unwrap();
    println!("{:#?}", tree.root_node().to_sexp());
}
