// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_plugins_output_next_token(
    input: &crate::output::ListPluginsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_templates_output_next_token(
    input: &crate::output::ListTemplatesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_template_step_groups_output_next_token(
    input: &crate::output::ListTemplateStepGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_template_steps_output_next_token(
    input: &crate::output::ListTemplateStepsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_workflows_output_next_token(
    input: &crate::output::ListWorkflowsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_workflow_step_groups_output_next_token(
    input: &crate::output::ListWorkflowStepGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_workflow_steps_output_next_token(
    input: &crate::output::ListWorkflowStepsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_plugins_output_plugins(
    input: crate::output::ListPluginsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::PluginSummary>> {
    let input = match input.plugins {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_templates_output_template_summary(
    input: crate::output::ListTemplatesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::TemplateSummary>> {
    let input = match input.template_summary {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_template_step_groups_output_template_step_group_summary(
    input: crate::output::ListTemplateStepGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::TemplateStepGroupSummary>> {
    let input = match input.template_step_group_summary {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_template_steps_output_template_step_summary_list(
    input: crate::output::ListTemplateStepsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::TemplateStepSummary>> {
    let input = match input.template_step_summary_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_workflows_output_migration_workflow_summary(
    input: crate::output::ListWorkflowsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::MigrationWorkflowSummary>> {
    let input = match input.migration_workflow_summary {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_workflow_step_groups_output_workflow_step_groups_summary(
    input: crate::output::ListWorkflowStepGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::WorkflowStepGroupSummary>> {
    let input = match input.workflow_step_groups_summary {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_workflow_steps_output_workflow_steps_summary(
    input: crate::output::ListWorkflowStepsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::WorkflowStepSummary>> {
    let input = match input.workflow_steps_summary {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
