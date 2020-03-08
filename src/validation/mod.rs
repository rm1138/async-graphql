use crate::error::RuleErrors;
use crate::registry::Registry;
use crate::validation::context::ValidatorContext;
use crate::validation::visitor::{visit, VisitorNil};
use crate::Result;
use graphql_parser::query::Document;

mod context;
mod rules;
mod utils;
mod visitor;

pub fn check_rules(registry: &Registry, doc: &Document) -> Result<()> {
    let mut ctx = ValidatorContext::new(registry);
    let mut visitor = VisitorNil
        .with(rules::ArgumentsOfCorrectType::default())
        .with(rules::DefaultValuesOfCorrectType);

    visit(&mut visitor, &mut ctx, doc);
    if !ctx.errors.is_empty() {
        Err(RuleErrors { errors: ctx.errors }.into())
    } else {
        Ok(())
    }
}
