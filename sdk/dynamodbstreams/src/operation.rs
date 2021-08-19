// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p>
/// <note>
/// <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p>
/// </note>
/// <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the
/// <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no
/// <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream
/// records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code>
/// are present, then that shard is closed and can no longer receive more data.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeStream {
    _private: (),
}
impl DescribeStream {
    /// Creates a new builder-style object to manufacture [`DescribeStreamInput`](crate::input::DescribeStreamInput)
    pub fn builder() -> crate::input::describe_stream_input::Builder {
        crate::input::describe_stream_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeStream {
    type Output =
        std::result::Result<crate::output::DescribeStreamOutput, crate::error::DescribeStreamError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_stream_error(response)
        } else {
            crate::operation_deser::parse_describe_stream_response(response)
        }
    }
}

/// <p>Retrieves the stream records from a given shard.</p>
/// <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator
/// specifies the position in the shard from which you want to start reading stream records
/// sequentially. If there are no stream records available in the portion of the shard that the
/// iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take
/// multiple calls to get to a portion of the shard that contains stream records.</p>
/// <note>
/// <p>
/// <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records,
/// whichever comes first.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRecords {
    _private: (),
}
impl GetRecords {
    /// Creates a new builder-style object to manufacture [`GetRecordsInput`](crate::input::GetRecordsInput)
    pub fn builder() -> crate::input::get_records_input::Builder {
        crate::input::get_records_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetRecords {
    type Output =
        std::result::Result<crate::output::GetRecordsOutput, crate::error::GetRecordsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_records_error(response)
        } else {
            crate::operation_deser::parse_get_records_response(response)
        }
    }
}

/// <p>Returns a shard iterator. A shard iterator provides information
/// about how to retrieve the stream records from within a shard.  Use
/// the shard iterator in a subsequent
/// <code>GetRecords</code> request to read the stream records
/// from the shard.</p>
/// <note>
/// <p>A shard iterator expires 15 minutes after it is returned to the requester.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetShardIterator {
    _private: (),
}
impl GetShardIterator {
    /// Creates a new builder-style object to manufacture [`GetShardIteratorInput`](crate::input::GetShardIteratorInput)
    pub fn builder() -> crate::input::get_shard_iterator_input::Builder {
        crate::input::get_shard_iterator_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetShardIterator {
    type Output = std::result::Result<
        crate::output::GetShardIteratorOutput,
        crate::error::GetShardIteratorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_shard_iterator_error(response)
        } else {
            crate::operation_deser::parse_get_shard_iterator_response(response)
        }
    }
}

/// <p>Returns an array of stream ARNs associated with the current account and endpoint. If the
/// <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the
/// streams ARNs for that table.</p>
/// <note>
/// <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListStreams {
    _private: (),
}
impl ListStreams {
    /// Creates a new builder-style object to manufacture [`ListStreamsInput`](crate::input::ListStreamsInput)
    pub fn builder() -> crate::input::list_streams_input::Builder {
        crate::input::list_streams_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListStreams {
    type Output =
        std::result::Result<crate::output::ListStreamsOutput, crate::error::ListStreamsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_streams_error(response)
        } else {
            crate::operation_deser::parse_list_streams_response(response)
        }
    }
}