pub struct GramAnalysisResult<T> {
    pub ast: T,
    pub next_cursor: usize,
}

enum SourceType {
    script,
    module,
}

impl SourceType {
    const script: &'static str = "script";
    const module: &'static str = "module";
}

enum Kind {
    init,
    get,
    set,
}

impl Kind {
    const init: &'static str = "init";
    const get: &'static str = "get";
    const set: &'static str = "set";
}

enum PropertyKey {
    Identifier(Identifier),
    ComputedPropertyKey(ComputedPropertyKey),
}
pub enum ASTType {
    Program,
    VariableDeclaration,
    VariableDeclarator,
    Identifier,
    ArrayExpression,
    ObjectExpression,
    Literal,
    FunctionDeclaration,
}

impl ASTType {
    const Program: &'static str = "Program";
    const VariableDeclaration: &'static str = "VariableDeclaration";
    const VariableDeclarator: &'static str = "VariableDeclarator";
    const Identifier: &'static str = "Identifier";
    const ArrayExpression: &'static str = "ArrayExpression";
    const ObjectExpression: &'static str = "ObjectExpression";
    const Literal: &'static str = "Literal";
    const FunctionDeclaration: &'static str = "FunctionDeclaration";
}

pub enum AST {
    Program(Program),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarator(VariableDeclarator),
    Identifier(Identifier),
    ArrayExpression(ArrayExpression),
}

enum Body {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ExpressionStatement(ExpressionStatement),
}

enum FunctionParams {
    Identifier(Identifier),
    AssignmentPattern(AssignmentPattern),
}

enum AssignmentPatternRight {
    Literal(Literal),
    Identifier(Identifier),
    MemberExpression(MemberExpression),
    CallExpression(Option<Box<CallExpression>>),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
}

enum MemberExpressionObject {
    Identifier(Identifier),
    MemberExpression(Option<Box<MemberExpression>>),
    CallExpression(Option<Box<CallExpression>>),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
}

enum MemberExpressionProperty {
    Identifier(Identifier),
    Literal(Literal),
    CallExpression(Option<Box<CallExpression>>),
    ObjectExpression(ObjectExpression),
    ArrayExpression(ArrayExpression),
    MemberExpression(Option<Box<MemberExpression>>),
}

enum CallExpressionCallee {
    Identifier(Identifier),
    MemberExpression(Option<Box<MemberExpression>>),
    CallExpression(Option<Box<CallExpression>>),
}

enum CallExpressionArgument {
    Identifier(Identifier),
    Literal(Literal),
    MemberExpression(Option<Box<MemberExpression>>),
    CallExpression(Option<Box<CallExpression>>),
    ObjectExpression(ObjectExpression),
    ArrayExpression(ArrayExpression),
}

pub enum Expression {
    Identifier(Identifier),
    ConditionalExpression(Option<Box<ConditionalExpression>>),
    BinaryExpression(Option<Box<BinaryExpression>>),
    CallExpression(Option<Box<CallExpression>>),
    MemberExpression(Option<Box<MemberExpression>>),
    ArrayExpression(Option<Box<ArrayExpression>>),
    Literal(Literal),
    NewExpression(Option<Box<NewExpression>>),
}

struct Program {
    _type: ASTType,
    body: Vec<Body>,
    sourceType: SourceType,
}

pub struct Identifier {
    pub _type: ASTType,
    pub start: u32,
    pub end: u32,
    pub name: String,
}

pub struct VariableDeclarator {
    pub _type: ASTType,
    pub start: u32,
    pub end: u32,
    pub id: Identifier,
    pub init: Expression,
}

struct VariableDeclaration {
    _type: ASTType,
    start: u32,
    end: u32,
    declarations: Vec<VariableDeclarator>,
}

struct ArrayExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    elements: Vec<Expression>,
}

struct ObjectExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    properties: Vec<Property>,
}

struct Property {
    _type: ASTType,
    start: u32,
    end: u32,
    method: bool,
    shorthand: bool,
    computed: bool,
    key: PropertyKey,
    value: Expression,
    kind: Kind,
}

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

pub struct Regex {
    pub pattern: &'static str,
    pub flags: &'static str,
}

pub struct Literal {
    pub _type: ASTType,
    pub start: u32,
    pub end: u32,
    pub value: LiteralType,
    pub raw: &'static str,
    pub regex: Option<Regex>,
}

struct ComputedPropertyKey {
    _type: ASTType,
    expression: Expression,
}

struct FunctionDeclaration {
    _type: ASTType,
    start: u32,
    end: u32,
    id: Identifier,
    expression: bool,
    generator: bool,
    _async: bool,
    params: Vec<FunctionParams>,
    body: BlockStatement,
}

struct BlockStatement {
    _type: ASTType,
    start: u32,
    end: u32,
    body: Vec<Body>,
}

struct AssignmentPattern {
    _type: ASTType,
    start: u32,
    end: u32,
    left: Identifier,
    right: AssignmentPatternRight,
}

struct MemberExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    object: Expression,
    property: Expression,
    computed: bool,
    optional: bool,
}

struct CallExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    callee: Expression,
    arguments: Vec<Expression>,
    optional: bool,
}

struct ExpressionStatement {
    _type: ASTType,
    start: u32,
    end: u32,
    expression: Expression,
}

struct ConditionalExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    test: Expression,
    consequent: Expression,
    alternate: Expression,
}

struct BinaryExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    left: Expression,
    operator: &'static str,
    right: Expression,
}

struct NewExpression {
    _type: ASTType,
    start: u32,
    end: u32,
    callee: Expression,
    arguments: Vec<Expression>,
}
