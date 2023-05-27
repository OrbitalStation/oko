use oko::*;

// TODO: Merge these repeats into one entity with metapolicies
// TODO: Generalize enum and struct in derive_parseable
// TODO: Add `ParseDebug`
// TODO: Make comments replace their content with ` ` to preserve span

fn main() {
    let ast = AST::new();
    let ast = ast.advance_with::<RawCodeASTStage>("code").unwrap();
    let ast = ast.advance::<SynASTState>().unwrap();
    let ast = ast.advance::<ScopeASTState>().unwrap();

    println!("\n{ast:#?}")
}
