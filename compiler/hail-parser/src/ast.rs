use std::ops::Range;

/// Wraps a node with debug/diagnostic information.
#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    /// The location of the node, in the source string.
    pub loc: Range<usize>,

    /// The node.
    pub node: T,
}

/// An integer node.
#[derive(Clone, Debug, PartialEq)]
pub struct Int<'a> {
    /// The value of the integer.
    pub value: &'a str,
}

/// A float node.
#[derive(Clone, Debug, PartialEq)]
pub struct Float<'a> {
    /// The value of the float.
    pub value: &'a str,
}

/// An identifier node.
#[derive(Clone, Debug, PartialEq)]
pub struct Iden<'a> {
    /// The value of the identifier.
    pub value: &'a str,
}

/// A string node.
#[derive(Clone, Debug, PartialEq)]
pub struct Str<'a> {
    /// The value of the string.
    pub value: &'a str,
}

/// An instruction node.
#[derive(Clone, Debug, PartialEq)]
pub enum InstNode<'a> {
    /// An integer node.
    Int(Int<'a>),

    /// A float node.
    Float(Float<'a>),

    /// An access node.
    AccessExpr(AccessExpr<'a>),

    /// A string node.
    Str(Str<'a>),
}

/// An access node.
#[derive(Clone, Debug, PartialEq)]
pub enum AccessNode<'a> {
    /// An identifier node.
    Iden(Iden<'a>),

    /// A static access.
    Static(Node<Box<AccessNode<'a>>>, Iden<'a>),
}

/// An access expression as a node.
/// 
/// ```hail
/// item::item2
/// // or
/// item.item2
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum AccessExpr<'a> {
    /// An identifier node.
    Iden(Iden<'a>),

    /// A static access.
    /// 
    /// ```hail
    /// item::item2
    /// ```
    Static(Node<Box<AccessExpr<'a>>>, Iden<'a>),
    
    /// A struct constructor.
    /// 
    /// ```hail
    /// item::{
    ///     // ....
    /// }
    /// ```
    Struct(Node<Box<AccessExpr<'a>>>, Vec<Node<StructNodeProp<'a>>>),

    /// A property access.
    /// 
    /// ```hail
    /// item.item2
    /// ```
    Property(Node<Box<AccessExpr<'a>>>, Iden<'a>),

    /// A generic type expression.
    Generic(Node<Box<AccessExpr<'a>>>, Vec<Node<TypeExpr<'a>>>),
}

/// The struct node property.
#[derive(Clone, Debug, PartialEq)]
pub struct StructNodeProp<'a> {
    /// An identifier token.
    pub name: Iden<'a>,

    /// The value of the property.
    pub value: Option<NodeOrNodes<'a>>,
}

/// One or more nodes.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeOrNodes<'a> {
    /// A single node.
    Node(InstNode<'a>),

    /// More than one node.
    Nodes(Vec<Node<InstNode<'a>>>),
}

/// A type expression.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExpr<'a> {
    Fluid(Node<Box<TypeExpr<'a>>>),
    Ptr(Node<Box<TypeExpr<'a>>>),
    Ref(Node<Box<TypeExpr<'a>>>),
    AccessNode(AccessNode<'a>),
    Generic(Node<AccessNode<'a>>, Vec<Node<TypeExpr<'a>>>),
    Struct(Node<Struct<'a>>),
    Enum(Node<Enum<'a>>),
}

/// A type declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl<'a> {
    /// The name of the type declaration.
    pub name: Node<Iden<'a>>,

    /// The type of the declaration.
    pub ty: Option<Node<TypeDeclExpr<'a>>>
}

/// A type expression in a type declaration.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeDeclExpr<'a> {
    /// An access node.
    AccessNode(AccessNode<'a>),

    /// A generic type expr.
    Generic(Node<Box<TypeDeclExpr<'a>>>, Vec<Node<TypeExpr<'a>>>),

    /// Multiple trait objects in one.
    And(Node<Box<TypeDeclExpr<'a>>>, Node<Box<TypeDeclExpr<'a>>>),
}

/// A variable declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Val<'a> {
    /// The name of the variable.
    pub name: Node<Iden<'a>>,

    /// The optional type annotation of the variable.
    pub ty: Option<Node<TypeExpr<'a>>>,

    /// The optional value of the variable.
    pub value: Option<Node<NodeOrNodes<'a>>>,
}

/// An if statement.
#[derive(Clone, Debug, PartialEq)]
pub struct If<'a> {
    /// The condition of the if statement.
    pub cond: Node<NodeOrNodes<'a>>,

    /// The if action.
    pub block: Vec<Node<BlockExpr<'a>>>,

    /// The branches of the if statement.
    pub branches: Vec<Node<IfBranch<'a>>>,
}

/// A branch after an if statement.
#[derive(Clone, Debug, PartialEq)]
pub enum IfBranch<'a> {
    /// An else if statement.
    ElseIf(Node<NodeOrNodes<'a>>, Vec<Node<BlockExpr<'a>>>),

    /// An else statement.
    Else(Vec<Node<BlockExpr<'a>>>),
}

/// A while statement.
#[derive(Clone, Debug, PartialEq)]
pub struct While<'a> {
    /// The condition of the while statement.
    pub cond: Node<NodeOrNodes<'a>>,
    
    /// The while action.
    pub block: Vec<Node<BlockExpr<'a>>>,
}

/// A test statement.
#[derive(Clone, Debug, PartialEq)]
pub struct Test<'a> {
    /// The test subject, ahaha
    pub subject: Node<NodeOrNodes<'a>>,
    
    /// The arms in a match statement.
    pub block: Vec<Node<TestArm<'a>>>,
}

/// A test arm.
#[derive(Clone, Debug, PartialEq)]
pub struct TestArm<'a> {
    /// An identifier literal.
    pub name: Node<Iden<'a>>,

    /// The type of the arm, if any.
    pub ty: Option<Node<TypeExpr<'a>>>,

    /// The action of the test arm.
    pub action: Node<NodeOrNodes<'a>>,
}

/// A block expression.
#[derive(Clone, Debug, PartialEq)]
pub enum BlockExpr<'a> {
    /// A node expression.
    Node(NodeOrNodes<'a>),

    /// A variable definition.
    Val(Val<'a>),

    /// An if statement.
    If(If<'a>),

    /// A while statement.
    While(While<'a>),

    /// A test statement.
    Test(Test<'a>),
}

/// A routine declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Routine<'a> {
    /// The generic types of the routine.
    pub generics: Option<Vec<Node<TypeDecl<'a>>>>,

    /// The args defined for this routine.
    pub args: Vec<Node<RoutineArg<'a>>>,

    /// What the routine returns, if anything is declared.
    pub returns: Option<Node<TypeExpr<'a>>>,

    /// A block expression.
    pub block: Option<Vec<Node<BlockExpr<'a>>>>,
}

/// An argument in a routine.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutineArg<'a> {
    /// The name of the argument.
    pub name: Node<Iden<'a>>,
    
    /// The type of the argument.
    pub ty: Node<TypeExpr<'a>>,
}

/// An enum declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Enum<'a> {
    /// The generic types of the enum.
    pub generics: Option<Vec<Node<TypeDecl<'a>>>>,

    /// The properties of this struct.
    pub props: Vec<Node<EnumMember<'a>>>,
}

/// A member of an enum..
#[derive(Clone, Debug, PartialEq)]
pub struct EnumMember<'a> {
    /// The name of the struct property.
    pub name: Node<Iden<'a>>,

    /// The type of the struct.
    pub ty: Option<Node<TypeExpr<'a>>>,
}

/// A struct declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Struct<'a> {
    /// The generic types of the struct.
    pub generics: Option<Vec<Node<TypeDecl<'a>>>>,

    /// The properties of this struct.
    pub props: Vec<Node<StructProp<'a>>>,
}

/// A property of a struct.
#[derive(Clone, Debug, PartialEq)]
pub struct StructProp<'a> {
    /// The name of the struct property.
    pub name: Node<Iden<'a>>,

    /// The type of the struct.
    pub ty: Node<TypeExpr<'a>>,
}

/// The kind of an import.
#[derive(Clone, Debug, PartialEq)]
pub enum ImportKind<'a> {
    /// Importing a whole module.
    Module,

    /// Specific items,
    Specific(Vec<Node<AccessNode<'a>>>),

    /// Imports all module items as a certain alias.
    AllAs(Node<Iden<'a>>),
}

/// An import statement.
#[derive(Clone, Debug, PartialEq)]
pub struct Import<'a> {
    /// The name of the imported module.
    pub module: Node<Iden<'a>>,

    /// The kind of import this statement matches.
    pub kind: ImportKind<'a>,
}

/// A declaration.
#[derive(Clone, Debug, PartialEq)]
pub enum Decl<'a> {
    Routine(Node<Iden<'a>>, Node<Routine<'a>>),
    Struct(Node<Iden<'a>>, Node<Struct<'a>>),
    Enum(Node<Iden<'a>>, Node<Enum<'a>>),
    Import(Import<'a>),
}