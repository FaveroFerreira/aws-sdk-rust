// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_keyspaces_output_next_token(
    input: &crate::output::ListKeyspacesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_tables_output_next_token(
    input: &crate::output::ListTablesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_tags_for_resource_output_next_token(
    input: &crate::output::ListTagsForResourceOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_keyspaces_output_keyspaces(
    input: crate::output::ListKeyspacesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::KeyspaceSummary>> {
    let input = match input.keyspaces {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_tables_output_tables(
    input: crate::output::ListTablesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::TableSummary>> {
    let input = match input.tables {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_tags_for_resource_output_tags(
    input: crate::output::ListTagsForResourceOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Tag>> {
    let input = match input.tags {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}