// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(std::fmt::Debug)]
pub(crate) struct Handle<C = aws_hyper::DynConnector> {
    client: aws_hyper::Client<C>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client<C = aws_hyper::DynConnector> {
    handle: std::sync::Arc<Handle<C>>,
}
impl<C> Client<C> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf(crate::Config::builder().build())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl<C> Client<C>
where
    C: aws_hyper::SmithyConnector,
{
    pub fn create_outpost(&self) -> fluent_builders::CreateOutpost<C> {
        fluent_builders::CreateOutpost::new(self.handle.clone())
    }
    pub fn delete_outpost(&self) -> fluent_builders::DeleteOutpost<C> {
        fluent_builders::DeleteOutpost::new(self.handle.clone())
    }
    pub fn delete_site(&self) -> fluent_builders::DeleteSite<C> {
        fluent_builders::DeleteSite::new(self.handle.clone())
    }
    pub fn get_outpost(&self) -> fluent_builders::GetOutpost<C> {
        fluent_builders::GetOutpost::new(self.handle.clone())
    }
    pub fn get_outpost_instance_types(&self) -> fluent_builders::GetOutpostInstanceTypes<C> {
        fluent_builders::GetOutpostInstanceTypes::new(self.handle.clone())
    }
    pub fn list_outposts(&self) -> fluent_builders::ListOutposts<C> {
        fluent_builders::ListOutposts::new(self.handle.clone())
    }
    pub fn list_sites(&self) -> fluent_builders::ListSites<C> {
        fluent_builders::ListSites::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateOutpost<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::create_outpost_input::Builder,
    }
    impl<C> CreateOutpost<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateOutpostOutput,
            smithy_http::result::SdkError<crate::error::CreateOutpostError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the Outpost.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(input);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>The description of the Outpost.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(input);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The ID of the site.</p>
        pub fn site_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.site_id(input);
            self
        }
        pub fn set_site_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_site_id(input);
            self
        }
        /// <p>The Availability Zone.</p>
        pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone(input);
            self
        }
        pub fn set_availability_zone(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_availability_zone(input);
            self
        }
        /// <p>The ID of the Availability Zone.</p>
        pub fn availability_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone_id(input);
            self
        }
        pub fn set_availability_zone_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_availability_zone_id(input);
            self
        }
        /// <p>The tags to apply to the Outpost.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteOutpost<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::delete_outpost_input::Builder,
    }
    impl<C> DeleteOutpost<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteOutpostOutput,
            smithy_http::result::SdkError<crate::error::DeleteOutpostError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the Outpost.</p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.outpost_id(input);
            self
        }
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_outpost_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteSite<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::delete_site_input::Builder,
    }
    impl<C> DeleteSite<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteSiteOutput,
            smithy_http::result::SdkError<crate::error::DeleteSiteError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the site.</p>
        pub fn site_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.site_id(input);
            self
        }
        pub fn set_site_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_site_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetOutpost<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_outpost_input::Builder,
    }
    impl<C> GetOutpost<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetOutpostOutput,
            smithy_http::result::SdkError<crate::error::GetOutpostError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the Outpost.</p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.outpost_id(input);
            self
        }
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_outpost_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetOutpostInstanceTypes<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_outpost_instance_types_input::Builder,
    }
    impl<C> GetOutpostInstanceTypes<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetOutpostInstanceTypesOutput,
            smithy_http::result::SdkError<crate::error::GetOutpostInstanceTypesError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the Outpost.</p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.outpost_id(input);
            self
        }
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_outpost_id(input);
            self
        }
        /// <p>The pagination token.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum page size.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListOutposts<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_outposts_input::Builder,
    }
    impl<C> ListOutposts<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListOutpostsOutput,
            smithy_http::result::SdkError<crate::error::ListOutpostsError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The pagination token.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum page size.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>
        /// A filter for the lifecycle status of the Outpost.
        /// </p>
        /// <p> Filter values are case sensitive. If you specify multiple values for a filter, the values
        /// are joined with an <code>OR</code>, and the request returns all results that match any of the
        /// specified values. </p>
        pub fn life_cycle_status_filter(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.life_cycle_status_filter(inp);
            self
        }
        pub fn set_life_cycle_status_filter(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_life_cycle_status_filter(input);
            self
        }
        /// <p> A filter for the Availibility Zone (<code>us-east-1a</code>) of the Outpost. </p>
        /// <p> Filter values are case sensitive. If you specify multiple values for a filter, the values
        /// are joined with an <code>OR</code>, and the request returns all results that match any of the
        /// specified values. </p>
        pub fn availability_zone_filter(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone_filter(inp);
            self
        }
        pub fn set_availability_zone_filter(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_availability_zone_filter(input);
            self
        }
        /// <p>
        /// A filter for the AZ IDs (<code>use1-az1</code>) of the Outpost.
        /// </p>
        /// <p> Filter values are case sensitive. If you specify multiple values for a filter, the values
        /// are joined with an <code>OR</code>, and the request returns all results that match any of the
        /// specified values. </p>
        pub fn availability_zone_id_filter(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone_id_filter(inp);
            self
        }
        pub fn set_availability_zone_id_filter(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_availability_zone_id_filter(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListSites<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_sites_input::Builder,
    }
    impl<C> ListSites<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListSitesOutput,
            smithy_http::result::SdkError<crate::error::ListSitesError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The pagination token.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum page size.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListTagsForResource<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl<C> ListTagsForResource<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListTagsForResourceOutput,
            smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct TagResource<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl<C> TagResource<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::TagResourceOutput,
            smithy_http::result::SdkError<crate::error::TagResourceError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>The tags to add to the resource.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UntagResource<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl<C> UntagResource<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UntagResourceOutput,
            smithy_http::result::SdkError<crate::error::UntagResourceError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>The tag keys.</p>
        pub fn tag_keys(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(inp);
            self
        }
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
}