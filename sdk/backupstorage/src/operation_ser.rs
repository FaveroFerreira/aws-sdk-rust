// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_payload_notify_object_complete_input(
    payload: aws_smithy_http::byte_stream::ByteStream,
) -> Result<aws_smithy_http::byte_stream::ByteStream, aws_smithy_http::operation::BuildError> {
    Ok(payload)
}

pub fn serialize_payload_put_chunk_input(
    payload: aws_smithy_http::byte_stream::ByteStream,
) -> Result<aws_smithy_http::byte_stream::ByteStream, aws_smithy_http::operation::BuildError> {
    Ok(payload)
}

pub fn serialize_payload_put_object_input(
    payload: aws_smithy_http::byte_stream::ByteStream,
) -> Result<aws_smithy_http::byte_stream::ByteStream, aws_smithy_http::operation::BuildError> {
    Ok(payload)
}

pub fn serialize_operation_crate_operation_start_object(
    input: &crate::input::StartObjectInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_object_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}