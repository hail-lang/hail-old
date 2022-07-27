//! Toolkit for lowering a hail AST to hail HIR.

use hail_parser::ast;

/// A unit being lowered from AST to HIR.
pub struct HirLowerUnit<'a> {
    /// The directory that this unit is stored in.
    pub dir: String,

    /// The AST tree being compiled.
    pub ast: Vec<ast::RootStmnt<'a>>,
}

/// A context which HIR units are stored in.
pub struct HirLowerContext {
    /// The flags that are enabled.
    pub flags: Vec<String>,

    /// Search paths for libraries.
    pub libs: Vec<String>,
}

/// Loads the imports from a lowering unit.
fn hir_lower_imports<'a>(ctx: &mut HirLowerContext, unit: &HirLowerUnit<'a>) {
    'imports: for item in &unit.ast {
        match item {
            ast::RootStmnt::Import(flags, imp) => {
                // Make sure all the required flags are enabled.
                for flag in flags {
                    if ctx.flags.contains(&flag.name.value.into()) {
                        if flag.neg {
                            continue 'imports;
                        }
                    } else {
                        if flag.neg {
                            continue;
                        }

                        continue 'imports;
                    }
                }
                
                println!("IMPORT!!");


            },
            _ => {},
        }
    }
}

/// Lowers a hail AST to hail HIR.
pub fn hir_lower<'a>(ctx: &mut HirLowerContext, unit: &HirLowerUnit<'a>) {
    hir_lower_imports(ctx, unit);
}