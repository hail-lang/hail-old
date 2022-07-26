//! The AST expressions that can be found in hail units.

use std::ops::Range;

/// The location of an expression.
pub type Span = Range<usize>;

/// A boolean expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Bool {
    /// The span of the boolean.
    pub span: Span,

    /// The raw value of the boolean.
    pub value: bool,
}

/// An identifier expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Id<'a> {
    /// The span of the identifier.
    pub span: Span,

    /// The raw value of the identifier.
    pub value: &'a str,
}

/// The kind of a number.
#[derive(Clone, Debug, PartialEq)]
pub enum NumKind {
    Int,
    XInt,
    BInt,
    Float,
}

/// A number expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Num<'a> {
    /// The span of the number.
    pub span: Span,

    /// The kind of the number.
    pub kind: NumKind,

    /// The raw value of the number.
    pub value: &'a str,
}

/// A string expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Str<'a> {
    /// The span of the string.
    pub span: Span,

    /// The raw value of the string.
    pub value: &'a str,
}

/// A compiler marker.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker<'a> {
    /// The span of this marker.
    pub span: Span,

    /// The name of the marker.
    pub name: Id<'a>,
}

/// An argument in a routine.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutineArg<'a> {
    /// The span of the argument.
    pub span: Span,

    /// The name of the argument.
    pub name: Id<'a>,

    /// The type annotation of the argument.
    pub ty: Type<'a>,
}

/// A routine declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Routine<'a> {
    /// The span of the routine.
    pub span: Span,

    /// The name of the rout
    pub args: Vec<RoutineArg<'a>>,

    /// The value that this routine returns.
    pub returns: Option<Type<'a>>,

    /// A marker in the routine.
    pub markers: Vec<Marker<'a>>,

    /// The block of the routine.
    pub block: Block<'a>,
}

/// A path in a type expression.
#[derive(Clone, Debug, PartialEq)]
pub struct PathType<'a> {
    /// The span of the path.
    pub span: Span,

    /// The left side of the path.
    pub left: Box<Type<'a>>,

    /// The right side of the path.
    pub right: Id<'a>,
}

/// A routine type.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutineType<'a> {
    /// The location of this routine.
    pub span: Span,

    /// The arguments of this routine.
    pub args: Vec<Type<'a>>,

    /// The type that this routine returns.
    pub returns: Option<Box<Type<'a>>>,
}

/// A struct property.
#[derive(Clone, Debug, PartialEq)]
pub struct StructProp<'a> {
    /// The location of this property.
    pub span: Span,

    /// The value of the property.
    pub name: Id<'a>,

    /// The type of this property.
    pub ty: Type<'a>,
}

/// A struct type.
#[derive(Clone, Debug, PartialEq)]
pub struct StructType<'a> {
    /// The location of this struct.
    pub span: Span,

    /// The properties of this struct.
    pub props: Vec<StructProp<'a>>,
}

/// An enum property.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumProp<'a> {
    /// The location of this property.
    pub span: Span,

    /// The value of the property.
    pub name: Id<'a>,

    /// The type of this property.
    pub ty: Option<Type<'a>>,
}

/// An enum type.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumType<'a> {
    /// The location of this enum.
    pub span: Span,

    /// The properties of this enum.
    pub props: Vec<EnumProp<'a>>,
}

/// A shared type.
#[derive(Clone, Debug, PartialEq)]
pub struct SharedType<'a> {
    /// The location of this shared type.
    pub span: Span,

    /// The subject of this shared type.
    pub subject: Box<Type<'a>>,
}

/// A fluid type.
#[derive(Clone, Debug, PartialEq)]
pub struct FluidType<'a> {
    /// The location of this fluid type.
    pub span: Span,

    /// The subject of this fluid type.
    pub subject: Box<Type<'a>>,
}

/// A ref type.
#[derive(Clone, Debug, PartialEq)]
pub struct RefType<'a> {
    /// The location of this ref type.
    pub span: Span,

    /// The subject of this ref type.
    pub subject: Box<Type<'a>>,
}

/// An option type.
#[derive(Clone, Debug, PartialEq)]
pub struct OptType<'a> {
    /// The location of this option type.
    pub span: Span,

    /// The subject of this option type.
    pub subject: Box<Type<'a>>,
}

/// A result type.
#[derive(Clone, Debug, PartialEq)]
pub struct ResType<'a> {
    /// The location of this result type.
    pub span: Span,

    /// The left side of this result type.
    pub ok: Box<Type<'a>>,

    /// The right side of this result type.
    pub err: Box<Type<'a>>,
}

/// A type expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Type<'a> {
    Id(Id<'a>),
    Path(PathType<'a>),
    Routine(RoutineType<'a>),
    Struct(StructType<'a>),
    Enum(EnumType<'a>),
    Shared(SharedType<'a>),
    Fluid(FluidType<'a>),
    Ref(RefType<'a>),
    Opt(OptType<'a>),
    Res(ResType<'a>),
}

/// A path expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Path<'a> {
    /// The span of the path.
    pub span: Span,

    /// The left side of the path.
    pub left: Box<Expr<'a>>,

    /// The right side of this path.
    pub right: Id<'a>,
}

/// An access expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Access<'a> {
    /// The span of the access.
    pub span: Span,

    /// The left side of the access.
    pub left: Box<Expr<'a>>,

    /// The right side of this access.
    pub right: Id<'a>,
}

/// An call expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Call<'a> {
    /// The span of the call.
    pub span: Span,

    /// The left side of the call.
    pub left: Box<Expr<'a>>,

    /// The arguments of this call.
    pub args: Vec<Expr<'a>>,
}

/// Operators for unary expressions.
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Min,
    Star,
    Bang,
    Borrow,
    Fluid,
    Shared,
}

/// A unary expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Unary<'a> {
    /// The span of this expression.
    pub span: Span,

    /// The operator of this expression.
    pub op: UnaryOp,

    /// The subject of this expression.
    pub subject: Box<Expr<'a>>,
}

/// An `as` expression.
#[derive(Clone, Debug, PartialEq)]
pub struct As<'a> {
    /// The location of the expressions.
    pub span: Span,

    /// The subject of the expression.
    pub subject: Box<Expr<'a>>,

    /// The type of the expression.
    pub ty: Type<'a>,
}

/// The binary operator.
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Star,
    Slash,
    Perc,
    Plus,
    Min,
    LtLt,
    GtGt,
    Amp,
    Caret,
    Pipe,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    AmpAmp,
    PipePipe,
}

/// A binary expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Binary<'a> {
    /// The operator of the expression.
    pub op: BinaryOp,

    /// The left side of the expression.
    pub left: Box<Expr<'a>>,

    /// The right side of the expression.
    pub right: Box<Expr<'a>>,
}

/// A property in a struct constructor.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructProp<'a> {
    /// The span of the property.
    pub span: Span,

    /// The name of the property.
    pub name: Id<'a>,

    /// The value of the property.
    pub value: Expr<'a>,
}

/// A struct constructor.
#[derive(Clone, Debug, PartialEq)]
pub struct Construct<'a> {
    /// The span of the construct.
    pub span: Span,

    /// The name of the construct.
    pub subject: Box<Expr<'a>>,

    /// The value of the construct.
    pub items: Vec<ConstructProp<'a>>,
}

/// An enum constructor.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructEnum<'a> {
    /// The span of the enum.
    pub span: Span,

    /// The name of the enum.
    pub subject: Box<Expr<'a>>,

    /// The properties of the enum.
    pub item: Box<Expr<'a>>,
}

/// An expression from hail source.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'a> {
    Bool(Bool),
    Id(Id<'a>),
    Num(Num<'a>),
    Str(Str<'a>),
    Path(Path<'a>),
    Access(Access<'a>),
    Call(Call<'a>),
    Unary(Unary<'a>),
    As(As<'a>),
    Binary(Binary<'a>),
    Routine(Routine<'a>),
    Construct(Construct<'a>),
    ConstructEnum(ConstructEnum<'a>),
}

/// The operator of an assignment expression.
#[derive(Clone, Debug, PartialEq)]
pub enum AssignOp {
    Eq,
    PlusEq,
    MinEq,
    StarEq,
    SlashEq,
    PercEq,
    AmpEq,
    PipeEq,
    CaretEq,
    LtLtEq,
    GtGtEq,
}

/// An assignment expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Assign<'a> {
    /// The span of the expression.
    pub span: Span,

    /// The assignment operator to use.
    pub op: AssignOp,

    /// The subject to assign to.
    pub left: Expr<'a>,

    /// The right side of the expression.
    pub right: Expr<'a>,
}

/// A variable declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Val<'a> {
    /// The span of the variable.
    pub span: Span,

    /// The name of the variable.
    pub name: Id<'a>,

    /// The type annotation of the variable.
    pub ty: Option<Type<'a>>,

    /// The value of the variable.
    pub value: Option<Expr<'a>>,
}

/// A type declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl<'a> {
    /// The span of the type declaration.
    pub span: Span,

    /// The name of the type declaration.
    pub name: Id<'a>,

    /// The value of the type declaration.
    pub value: Option<Type<'a>>,
}

/// A branch after an `if` statement.
#[derive(Clone, Debug, PartialEq)]
pub enum IfBranch<'a> {
    /// An `else if` statement.
    ElseIf {
        /// The span of the `if` branch.
        span: Span,

        /// The condition of the `if` branch.
        cond: Expr<'a>,

        /// The block of the `if` branch.
        block: Block<'a>,
    },

    /// An `else` statement.
    Else {
        /// The span of the `if` branch.
        span: Span,

        /// The block of the `if` branch.
        block: Block<'a>,
    },
}

/// An `if` statement.
#[derive(Clone, Debug, PartialEq)]
pub struct If<'a> {
    /// The span of the statement.
    pub span: Span,

    /// The condition of the statement.
    pub cond: Expr<'a>,

    /// The block of the statement.
    pub block: Block<'a>,

    /// The branches of the statement.
    pub branches: Vec<IfBranch<'a>>,
}

/// A `while` statement.
#[derive(Clone, Debug, PartialEq)]
pub struct While<'a> {
    /// The span of the statement.
    pub span: Span,

    /// The label of the statement.
    pub label: Option<Id<'a>>,

    /// The condition of the statement.
    pub cond: Expr<'a>,

    /// The block of the statement.
    pub block: Block<'a>,
}

/// A case in a match statement.
#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase<'a> {
    /// The span of the case.
    pub span: Span,

    /// The name of the case.
    pub name: Id<'a>,

    /// The type of the case.
    pub ty: Type<'a>,

    /// The block of the case.
    pub block: Block<'a>,
}

/// A `match` statement.
#[derive(Clone, Debug, PartialEq)]
pub struct Match<'a> {
    /// The span of the statement.
    pub span: Span,

    /// The subject of the statement.
    pub subject: Expr<'a>,

    /// The cases of the statement.
    pub cases: Vec<MatchCase<'a>>,
}

/// An expression in a block.
#[derive(Clone, Debug, PartialEq)]
pub enum BlockExpr<'a> {
    Assign(Assign<'a>),
    Val(Val<'a>),
    Call(Call<'a>),
    If(If<'a>),
    While(While<'a>),
    Match(Match<'a>),
    TypeDecl(TypeDecl<'a>),
}

/// A code block.
#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    /// The location of the block.
    pub span: Span,

    /// The items in the block.
    pub items: Vec<BlockExpr<'a>>,
}

/// An item being imported.
#[derive(Clone, Debug, PartialEq)]
pub struct ImportItem<'a> {
    /// The span of the item.
    pub span: Span,

    /// The name of the item.
    pub name: Id<'a>,

    /// The alias to use, if any.
    pub as_: Option<Id<'a>>,
}

/// An import statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Import<'a> {
    /// A single item import.
    Single {
        /// The location of the import.
        span: Span,

        /// Whether or not the import is shared.
        shared: bool,

        /// The name of the item to import.
        item: ImportItem<'a>,

        /// The module that the item is from.
        from: Option<Id<'a>>,
    },

    /// A single item import.
    Multi {
        /// The location of the import.
        span: Span,

        /// Whether or not the import is shared.
        shared: bool,

        /// The name of the item to import.
        items: Vec<ImportItem<'a>>,

        /// The module that the item is from.
        from: Id<'a>,
    },
}

/// A statement in the root of a hail unit.
#[derive(Clone, Debug, PartialEq)]
pub enum RootStmnt<'a> {
    Assign(Assign<'a>),
    Val(Val<'a>),
    Call(Call<'a>),
    Import(Import<'a>),
    If(If<'a>),
    While(While<'a>),
    Match(Match<'a>),
    TypeDecl(TypeDecl<'a>),
}