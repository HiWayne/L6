#[derive(Debug)]
pub struct GramAnalysisResult<T> {
    pub ast: T,
    pub next_cursor: usize,
}

#[derive(Debug)]
pub enum SourceType {
    script,
    module,
}

impl SourceType {
    const script: &'static str = "script";
    const module: &'static str = "module";
}

#[derive(Debug)]
pub enum Kind {
    Init,
    Get,
    Set,
}

impl Kind {
    const Init: &'static str = "init";
    const Get: &'static str = "get";
    const Set: &'static str = "set";
}

#[derive(Debug)]
pub enum PropertyKey<'a> {
    Identifier(Identifier),
    ComputedPropertyKey(Expression<'a>),
}
#[derive(Debug)]
pub enum ASTType {
    Program,
    VariableDeclaration,
    VariableDeclarator,
    Identifier,
    ArrayExpression,
    ObjectExpression,
    FunctionExpression,
    ArrowFunctionExpression,
    Literal,
    FunctionDeclaration,
    BlockStatement,
    Property,
}

impl ASTType {
    const Program: &'static str = "Program";
    const VariableDeclaration: &'static str = "VariableDeclaration";
    const VariableDeclarator: &'static str = "VariableDeclarator";
    const Identifier: &'static str = "Identifier";
    const ArrayExpression: &'static str = "ArrayExpression";
    const ObjectExpression: &'static str = "ObjectExpression";
    const FunctionExpression: &'static str = "FunctionExpression";
    const ArrowFunctionExpression: &'static str = "ArrowFunctionExpression";
    const Literal: &'static str = "Literal";
    const FunctionDeclaration: &'static str = "FunctionDeclaration";
    const BlockStatement: &'static str = "BlockStatement";
    const Property: &'static str = "Property";
}

#[derive(Debug)]
pub enum AST<'a> {
    Program(Program<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    VariableDeclarator(VariableDeclarator<'a>),
    Identifier(Identifier),
    ArrayExpression(ArrayExpression<'a>),
}

#[derive(Debug)]
pub enum Body<'a> {
    VariableDeclaration(VariableDeclaration<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
    ExpressionStatement(ExpressionStatement<'a>),
    BlockStatement(BlockStatement<'a>),
}

#[derive(Debug)]
enum FunctionParams<'a> {
    Identifier(Identifier),
    AssignmentPattern(AssignmentPattern<'a>),
}

#[derive(Debug)]
enum AssignmentPatternRight<'a> {
    Literal(Literal<'a>),
    Identifier(Identifier),
    MemberExpression(MemberExpression<'a>),
    CallExpression(Option<Box<CallExpression<'a>>>),
    ArrayExpression(ArrayExpression<'a>),
    ObjectExpression(ObjectExpression<'a>),
}

#[derive(Debug)]
enum MemberExpressionObject<'a> {
    Identifier(Identifier),
    MemberExpression(Option<Box<MemberExpression<'a>>>),
    CallExpression(Option<Box<CallExpression<'a>>>),
    ArrayExpression(ArrayExpression<'a>),
    ObjectExpression(ObjectExpression<'a>),
}

#[derive(Debug)]
enum MemberExpressionProperty<'a> {
    Identifier(Identifier),
    Literal(Literal<'a>),
    CallExpression(Option<Box<CallExpression<'a>>>),
    ObjectExpression(ObjectExpression<'a>),
    ArrayExpression(ArrayExpression<'a>),
    MemberExpression(Option<Box<MemberExpression<'a>>>),
}

#[derive(Debug)]
enum CallExpressionCallee<'a> {
    Identifier(Identifier),
    MemberExpression(Option<Box<MemberExpression<'a>>>),
    CallExpression(Option<Box<CallExpression<'a>>>),
}

#[derive(Debug)]
enum CallExpressionArgument<'a> {
    Identifier(Identifier),
    Literal(Literal<'a>),
    MemberExpression(Option<Box<MemberExpression<'a>>>),
    CallExpression(Option<Box<CallExpression<'a>>>),
    ObjectExpression(ObjectExpression<'a>),
    ArrayExpression(ArrayExpression<'a>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(Identifier),
    ConditionalExpression(Option<Box<ConditionalExpression<'a>>>),
    BinaryExpression(Option<Box<BinaryExpression<'a>>>),
    CallExpression(Option<Box<CallExpression<'a>>>),
    MemberExpression(Option<Box<MemberExpression<'a>>>),
    ArrayExpression(Option<Box<ArrayExpression<'a>>>),
    Literal(Literal<'a>),
    NewExpression(Option<Box<NewExpression<'a>>>),
    FunctionExpression(FunctionExpression<'a>),
    ArrowFunctionExpression(ArrowFunctionExpression<'a>),
}

#[derive(Debug)]
pub struct Program<'a> {
    pub _type: ASTType,
    pub body: Vec<Body<'a>>,
    pub sourceType: SourceType,
}

#[derive(Debug)]
pub struct Identifier {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct VariableDeclarator<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub id: Identifier,
    pub init: Expression<'a>,
}

#[derive(Debug)]
pub enum DeclarationKind {
    Const,
    Let,
    Var,
}

impl DeclarationKind {
    const Const: &'static str = "const";
    const Let: &'static str = "let";
    const Var: &'static str = "var";
}

#[derive(Debug)]
pub struct VariableDeclaration<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub declarations: Vec<VariableDeclarator<'a>>,
    pub kind: DeclarationKind,
}

#[derive(Debug)]
pub struct ArrayExpression<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub elements: Vec<Expression<'a>>,
}

#[derive(Debug)]
struct ObjectExpression<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    properties: Vec<Property<'a>>,
}

#[derive(Debug)]
pub struct Property<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
    pub key: PropertyKey<'a>,
    pub value: Expression<'a>,
    pub kind: Kind,
}

#[derive(Debug)]
pub enum LiteralType {
    STRING,
    NUMBER,
    BOOLEAN,
    NULL,
    REGEXP,
}

impl LiteralType {
    const STRING: &'static str = "string";
    const NUMBER: &'static str = "number";
    const BOOLEAN: &'static str = "boolean";
    const NULL: &'static str = "null";
    const REGEXP: &'static str = "regexp";
}

#[derive(Debug)]
pub struct Regex<'a> {
    pub pattern: &'a str,
    pub flags: &'a str,
}

#[derive(Debug)]
pub struct Literal<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub value: LiteralType,
    pub raw: &'a str,
    pub regex: Option<Regex<'a>>,
}

#[derive(Debug)]
struct FunctionDeclaration<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    id: Identifier,
    expression: bool,
    generator: bool,
    _async: bool,
    params: Vec<FunctionParams<'a>>,
    body: BlockStatement<'a>,
}

#[derive(Debug)]
pub struct BlockStatement<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub body: Vec<Body<'a>>,
}

#[derive(Debug)]
struct AssignmentPattern<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    left: Identifier,
    right: AssignmentPatternRight<'a>,
}

#[derive(Debug)]
struct MemberExpression<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    object: Expression<'a>,
    property: Expression<'a>,
    computed: bool,
    optional: bool,
}

#[derive(Debug)]
struct CallExpression<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    callee: Expression<'a>,
    arguments: Vec<Expression<'a>>,
    optional: bool,
}

#[derive(Debug)]
struct ExpressionStatement<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    expression: Expression<'a>,
}

#[derive(Debug)]
struct ConditionalExpression<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    test: Expression<'a>,
    consequent: Expression<'a>,
    alternate: Expression<'a>,
}

#[derive(Debug)]
struct BinaryExpression<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    left: Expression<'a>,
    operator: &'static str,
    right: Expression<'a>,
}

#[derive(Debug)]
struct NewExpression<'a> {
    _type: ASTType,
    start: usize,
    end: usize,
    callee: Expression<'a>,
    arguments: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct FunctionExpression<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub id: Option<Identifier>,
    pub expression: bool,
    pub generator: bool,
    pub _async: bool,
    pub params: Vec<Identifier>,
    pub body: BlockStatement<'a>,
}

#[derive(Debug)]
pub struct ArrowFunctionExpression<'a> {
    pub _type: ASTType,
    pub start: usize,
    pub end: usize,
    pub id: Option<Identifier>,
    pub expression: bool,
    pub generator: bool,
    pub _async: bool,
    pub params: Vec<Identifier>,
    pub body: BlockStatement<'a>,
}
