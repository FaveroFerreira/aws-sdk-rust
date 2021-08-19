// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateEventIntegrationOutput {}
impl std::fmt::Debug for UpdateEventIntegrationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateEventIntegrationOutput");
        formatter.finish()
    }
}
/// See [`UpdateEventIntegrationOutput`](crate::output::UpdateEventIntegrationOutput)
pub mod update_event_integration_output {
    /// A builder for [`UpdateEventIntegrationOutput`](crate::output::UpdateEventIntegrationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateEventIntegrationOutput`](crate::output::UpdateEventIntegrationOutput)
        pub fn build(self) -> crate::output::UpdateEventIntegrationOutput {
            crate::output::UpdateEventIntegrationOutput {}
        }
    }
}
impl UpdateEventIntegrationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateEventIntegrationOutput`](crate::output::UpdateEventIntegrationOutput)
    pub fn builder() -> crate::output::update_event_integration_output::Builder {
        crate::output::update_event_integration_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput)
pub mod untag_resource_output {
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput)
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput)
pub mod tag_resource_output {
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput)
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput)
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p>Information about the tags.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
pub mod list_tags_for_resource_output {
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListEventIntegrationsOutput {
    /// <p>The event integrations.</p>
    pub event_integrations: std::option::Option<std::vec::Vec<crate::model::EventIntegration>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListEventIntegrationsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListEventIntegrationsOutput");
        formatter.field("event_integrations", &self.event_integrations);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListEventIntegrationsOutput`](crate::output::ListEventIntegrationsOutput)
pub mod list_event_integrations_output {
    /// A builder for [`ListEventIntegrationsOutput`](crate::output::ListEventIntegrationsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) event_integrations:
            std::option::Option<std::vec::Vec<crate::model::EventIntegration>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn event_integrations(
            mut self,
            input: impl Into<crate::model::EventIntegration>,
        ) -> Self {
            let mut v = self.event_integrations.unwrap_or_default();
            v.push(input.into());
            self.event_integrations = Some(v);
            self
        }
        pub fn set_event_integrations(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EventIntegration>>,
        ) -> Self {
            self.event_integrations = input;
            self
        }
        /// <p>If there are additional results, this is the token for the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEventIntegrationsOutput`](crate::output::ListEventIntegrationsOutput)
        pub fn build(self) -> crate::output::ListEventIntegrationsOutput {
            crate::output::ListEventIntegrationsOutput {
                event_integrations: self.event_integrations,
                next_token: self.next_token,
            }
        }
    }
}
impl ListEventIntegrationsOutput {
    /// Creates a new builder-style object to manufacture [`ListEventIntegrationsOutput`](crate::output::ListEventIntegrationsOutput)
    pub fn builder() -> crate::output::list_event_integrations_output::Builder {
        crate::output::list_event_integrations_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListEventIntegrationAssociationsOutput {
    /// <p>The event integration associations.</p>
    pub event_integration_associations:
        std::option::Option<std::vec::Vec<crate::model::EventIntegrationAssociation>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListEventIntegrationAssociationsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListEventIntegrationAssociationsOutput");
        formatter.field(
            "event_integration_associations",
            &self.event_integration_associations,
        );
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListEventIntegrationAssociationsOutput`](crate::output::ListEventIntegrationAssociationsOutput)
pub mod list_event_integration_associations_output {
    /// A builder for [`ListEventIntegrationAssociationsOutput`](crate::output::ListEventIntegrationAssociationsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) event_integration_associations:
            std::option::Option<std::vec::Vec<crate::model::EventIntegrationAssociation>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn event_integration_associations(
            mut self,
            input: impl Into<crate::model::EventIntegrationAssociation>,
        ) -> Self {
            let mut v = self.event_integration_associations.unwrap_or_default();
            v.push(input.into());
            self.event_integration_associations = Some(v);
            self
        }
        pub fn set_event_integration_associations(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EventIntegrationAssociation>>,
        ) -> Self {
            self.event_integration_associations = input;
            self
        }
        /// <p>If there are additional results, this is the token for the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEventIntegrationAssociationsOutput`](crate::output::ListEventIntegrationAssociationsOutput)
        pub fn build(self) -> crate::output::ListEventIntegrationAssociationsOutput {
            crate::output::ListEventIntegrationAssociationsOutput {
                event_integration_associations: self.event_integration_associations,
                next_token: self.next_token,
            }
        }
    }
}
impl ListEventIntegrationAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`ListEventIntegrationAssociationsOutput`](crate::output::ListEventIntegrationAssociationsOutput)
    pub fn builder() -> crate::output::list_event_integration_associations_output::Builder {
        crate::output::list_event_integration_associations_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetEventIntegrationOutput {
    /// <p>The name of the event integration. </p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The description of the event integration.</p>
    pub description: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the event integration.</p>
    pub event_integration_arn: std::option::Option<std::string::String>,
    /// <p>The EventBridge bus.</p>
    pub event_bridge_bus: std::option::Option<std::string::String>,
    /// <p>The event filter.</p>
    pub event_filter: std::option::Option<crate::model::EventFilter>,
    /// <p>One or more tags.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for GetEventIntegrationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetEventIntegrationOutput");
        formatter.field("name", &self.name);
        formatter.field("description", &self.description);
        formatter.field("event_integration_arn", &self.event_integration_arn);
        formatter.field("event_bridge_bus", &self.event_bridge_bus);
        formatter.field("event_filter", &self.event_filter);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`GetEventIntegrationOutput`](crate::output::GetEventIntegrationOutput)
pub mod get_event_integration_output {
    /// A builder for [`GetEventIntegrationOutput`](crate::output::GetEventIntegrationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) event_integration_arn: std::option::Option<std::string::String>,
        pub(crate) event_bridge_bus: std::option::Option<std::string::String>,
        pub(crate) event_filter: std::option::Option<crate::model::EventFilter>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The name of the event integration. </p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The description of the event integration.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) for the event integration.</p>
        pub fn event_integration_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_integration_arn = Some(input.into());
            self
        }
        pub fn set_event_integration_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.event_integration_arn = input;
            self
        }
        /// <p>The EventBridge bus.</p>
        pub fn event_bridge_bus(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_bridge_bus = Some(input.into());
            self
        }
        pub fn set_event_bridge_bus(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.event_bridge_bus = input;
            self
        }
        /// <p>The event filter.</p>
        pub fn event_filter(mut self, input: crate::model::EventFilter) -> Self {
            self.event_filter = Some(input);
            self
        }
        pub fn set_event_filter(
            mut self,
            input: std::option::Option<crate::model::EventFilter>,
        ) -> Self {
            self.event_filter = input;
            self
        }
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`GetEventIntegrationOutput`](crate::output::GetEventIntegrationOutput)
        pub fn build(self) -> crate::output::GetEventIntegrationOutput {
            crate::output::GetEventIntegrationOutput {
                name: self.name,
                description: self.description,
                event_integration_arn: self.event_integration_arn,
                event_bridge_bus: self.event_bridge_bus,
                event_filter: self.event_filter,
                tags: self.tags,
            }
        }
    }
}
impl GetEventIntegrationOutput {
    /// Creates a new builder-style object to manufacture [`GetEventIntegrationOutput`](crate::output::GetEventIntegrationOutput)
    pub fn builder() -> crate::output::get_event_integration_output::Builder {
        crate::output::get_event_integration_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteEventIntegrationOutput {}
impl std::fmt::Debug for DeleteEventIntegrationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteEventIntegrationOutput");
        formatter.finish()
    }
}
/// See [`DeleteEventIntegrationOutput`](crate::output::DeleteEventIntegrationOutput)
pub mod delete_event_integration_output {
    /// A builder for [`DeleteEventIntegrationOutput`](crate::output::DeleteEventIntegrationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteEventIntegrationOutput`](crate::output::DeleteEventIntegrationOutput)
        pub fn build(self) -> crate::output::DeleteEventIntegrationOutput {
            crate::output::DeleteEventIntegrationOutput {}
        }
    }
}
impl DeleteEventIntegrationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteEventIntegrationOutput`](crate::output::DeleteEventIntegrationOutput)
    pub fn builder() -> crate::output::delete_event_integration_output::Builder {
        crate::output::delete_event_integration_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateEventIntegrationOutput {
    /// <p>The Amazon Resource Name (ARN) of the event integration. </p>
    pub event_integration_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CreateEventIntegrationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateEventIntegrationOutput");
        formatter.field("event_integration_arn", &self.event_integration_arn);
        formatter.finish()
    }
}
/// See [`CreateEventIntegrationOutput`](crate::output::CreateEventIntegrationOutput)
pub mod create_event_integration_output {
    /// A builder for [`CreateEventIntegrationOutput`](crate::output::CreateEventIntegrationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) event_integration_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the event integration. </p>
        pub fn event_integration_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_integration_arn = Some(input.into());
            self
        }
        pub fn set_event_integration_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.event_integration_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateEventIntegrationOutput`](crate::output::CreateEventIntegrationOutput)
        pub fn build(self) -> crate::output::CreateEventIntegrationOutput {
            crate::output::CreateEventIntegrationOutput {
                event_integration_arn: self.event_integration_arn,
            }
        }
    }
}
impl CreateEventIntegrationOutput {
    /// Creates a new builder-style object to manufacture [`CreateEventIntegrationOutput`](crate::output::CreateEventIntegrationOutput)
    pub fn builder() -> crate::output::create_event_integration_output::Builder {
        crate::output::create_event_integration_output::Builder::default()
    }
}