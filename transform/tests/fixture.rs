use std::path::PathBuf;

use swc_core::{
    // common::{chain, Mark},
    ecma::{
        visit::as_folder,
        parser::{EsSyntax, Syntax},
        transforms::{
            // base::resolver,
            testing::{test_fixture, FixtureTestConfig},
        },
    },
};

use jsx_control_statements::visitor::JSXControlStatements;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn jsx_control_statements_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        // &|_| {
        //     chain!(
        //         resolver(Mark::new(), Mark::new(), false),
        //         as_folder(JSXControlStatements)
        //     )
        // },
        &|_| as_folder(JSXControlStatements),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            ..Default::default()
        },
    );
}
