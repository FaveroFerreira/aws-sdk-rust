// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetInput,
) {
    if let Some(var_1) = &input.dataset_name {
        object.key("DatasetName").string(var_1);
    }
    if let Some(var_2) = &input.dataset_schema {
        let mut object_3 = object.key("DatasetSchema").start_object();
        crate::json_ser::serialize_structure_dataset_schema(&mut object_3, var_2);
        object_3.finish();
    }
    if let Some(var_4) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_4);
    }
    if let Some(var_5) = &input.client_token {
        object.key("ClientToken").string(var_5);
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
            {
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_9, item_8);
                object_9.finish();
            }
        }
        array_7.finish();
    }
}

pub fn serialize_structure_create_inference_scheduler_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateInferenceSchedulerInput,
) {
    if let Some(var_10) = &input.model_name {
        object.key("ModelName").string(var_10);
    }
    if let Some(var_11) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_11);
    }
    if let Some(var_12) = &input.data_delay_offset_in_minutes {
        object.key("DataDelayOffsetInMinutes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.data_upload_frequency {
        object.key("DataUploadFrequency").string(var_13.as_str());
    }
    if let Some(var_14) = &input.data_input_configuration {
        let mut object_15 = object.key("DataInputConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_input_configuration(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.data_output_configuration {
        let mut object_17 = object.key("DataOutputConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_output_configuration(&mut object_17, var_16);
        object_17.finish();
    }
    if let Some(var_18) = &input.role_arn {
        object.key("RoleArn").string(var_18);
    }
    if let Some(var_19) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_19);
    }
    if let Some(var_20) = &input.client_token {
        object.key("ClientToken").string(var_20);
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_24, item_23);
                object_24.finish();
            }
        }
        array_22.finish();
    }
}

pub fn serialize_structure_create_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateModelInput,
) {
    if let Some(var_25) = &input.model_name {
        object.key("ModelName").string(var_25);
    }
    if let Some(var_26) = &input.dataset_name {
        object.key("DatasetName").string(var_26);
    }
    if let Some(var_27) = &input.dataset_schema {
        let mut object_28 = object.key("DatasetSchema").start_object();
        crate::json_ser::serialize_structure_dataset_schema(&mut object_28, var_27);
        object_28.finish();
    }
    if let Some(var_29) = &input.labels_input_configuration {
        let mut object_30 = object.key("LabelsInputConfiguration").start_object();
        crate::json_ser::serialize_structure_labels_input_configuration(&mut object_30, var_29);
        object_30.finish();
    }
    if let Some(var_31) = &input.client_token {
        object.key("ClientToken").string(var_31);
    }
    if let Some(var_32) = &input.training_data_start_time {
        object
            .key("TrainingDataStartTime")
            .instant(var_32, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_33) = &input.training_data_end_time {
        object
            .key("TrainingDataEndTime")
            .instant(var_33, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_34) = &input.evaluation_data_start_time {
        object
            .key("EvaluationDataStartTime")
            .instant(var_34, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_35) = &input.evaluation_data_end_time {
        object
            .key("EvaluationDataEndTime")
            .instant(var_35, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_36) = &input.role_arn {
        object.key("RoleArn").string(var_36);
    }
    if let Some(var_37) = &input.data_pre_processing_configuration {
        let mut object_38 = object.key("DataPreProcessingConfiguration").start_object();
        crate::json_ser::serialize_structure_data_pre_processing_configuration(
            &mut object_38,
            var_37,
        );
        object_38.finish();
    }
    if let Some(var_39) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_39);
    }
    if let Some(var_40) = &input.tags {
        let mut array_41 = object.key("Tags").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_43, item_42);
                object_43.finish();
            }
        }
        array_41.finish();
    }
}

pub fn serialize_structure_delete_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDatasetInput,
) {
    if let Some(var_44) = &input.dataset_name {
        object.key("DatasetName").string(var_44);
    }
}

pub fn serialize_structure_delete_inference_scheduler_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteInferenceSchedulerInput,
) {
    if let Some(var_45) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_45);
    }
}

pub fn serialize_structure_delete_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteModelInput,
) {
    if let Some(var_46) = &input.model_name {
        object.key("ModelName").string(var_46);
    }
}

pub fn serialize_structure_describe_data_ingestion_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDataIngestionJobInput,
) {
    if let Some(var_47) = &input.job_id {
        object.key("JobId").string(var_47);
    }
}

pub fn serialize_structure_describe_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDatasetInput,
) {
    if let Some(var_48) = &input.dataset_name {
        object.key("DatasetName").string(var_48);
    }
}

pub fn serialize_structure_describe_inference_scheduler_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeInferenceSchedulerInput,
) {
    if let Some(var_49) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_49);
    }
}

pub fn serialize_structure_describe_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeModelInput,
) {
    if let Some(var_50) = &input.model_name {
        object.key("ModelName").string(var_50);
    }
}

pub fn serialize_structure_list_data_ingestion_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDataIngestionJobsInput,
) {
    if let Some(var_51) = &input.dataset_name {
        object.key("DatasetName").string(var_51);
    }
    if let Some(var_52) = &input.next_token {
        object.key("NextToken").string(var_52);
    }
    if let Some(var_53) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_53).into()),
        );
    }
    if let Some(var_54) = &input.status {
        object.key("Status").string(var_54.as_str());
    }
}

pub fn serialize_structure_list_datasets_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatasetsInput,
) {
    if let Some(var_55) = &input.next_token {
        object.key("NextToken").string(var_55);
    }
    if let Some(var_56) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    if let Some(var_57) = &input.dataset_name_begins_with {
        object.key("DatasetNameBeginsWith").string(var_57);
    }
}

pub fn serialize_structure_list_inference_executions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListInferenceExecutionsInput,
) {
    if let Some(var_58) = &input.next_token {
        object.key("NextToken").string(var_58);
    }
    if let Some(var_59) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_59).into()),
        );
    }
    if let Some(var_60) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_60);
    }
    if let Some(var_61) = &input.data_start_time_after {
        object
            .key("DataStartTimeAfter")
            .instant(var_61, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_62) = &input.data_end_time_before {
        object
            .key("DataEndTimeBefore")
            .instant(var_62, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_63) = &input.status {
        object.key("Status").string(var_63.as_str());
    }
}

pub fn serialize_structure_list_inference_schedulers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListInferenceSchedulersInput,
) {
    if let Some(var_64) = &input.next_token {
        object.key("NextToken").string(var_64);
    }
    if let Some(var_65) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_65).into()),
        );
    }
    if let Some(var_66) = &input.inference_scheduler_name_begins_with {
        object
            .key("InferenceSchedulerNameBeginsWith")
            .string(var_66);
    }
    if let Some(var_67) = &input.model_name {
        object.key("ModelName").string(var_67);
    }
}

pub fn serialize_structure_list_models_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListModelsInput,
) {
    if let Some(var_68) = &input.next_token {
        object.key("NextToken").string(var_68);
    }
    if let Some(var_69) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_69).into()),
        );
    }
    if let Some(var_70) = &input.status {
        object.key("Status").string(var_70.as_str());
    }
    if let Some(var_71) = &input.model_name_begins_with {
        object.key("ModelNameBeginsWith").string(var_71);
    }
    if let Some(var_72) = &input.dataset_name_begins_with {
        object.key("DatasetNameBeginsWith").string(var_72);
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_73) = &input.resource_arn {
        object.key("ResourceArn").string(var_73);
    }
}

pub fn serialize_structure_start_data_ingestion_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDataIngestionJobInput,
) {
    if let Some(var_74) = &input.dataset_name {
        object.key("DatasetName").string(var_74);
    }
    if let Some(var_75) = &input.ingestion_input_configuration {
        let mut object_76 = object.key("IngestionInputConfiguration").start_object();
        crate::json_ser::serialize_structure_ingestion_input_configuration(&mut object_76, var_75);
        object_76.finish();
    }
    if let Some(var_77) = &input.role_arn {
        object.key("RoleArn").string(var_77);
    }
    if let Some(var_78) = &input.client_token {
        object.key("ClientToken").string(var_78);
    }
}

pub fn serialize_structure_start_inference_scheduler_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartInferenceSchedulerInput,
) {
    if let Some(var_79) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_79);
    }
}

pub fn serialize_structure_stop_inference_scheduler_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopInferenceSchedulerInput,
) {
    if let Some(var_80) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_80);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_81) = &input.resource_arn {
        object.key("ResourceArn").string(var_81);
    }
    if let Some(var_82) = &input.tags {
        let mut array_83 = object.key("Tags").start_array();
        for item_84 in var_82 {
            {
                let mut object_85 = array_83.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_85, item_84);
                object_85.finish();
            }
        }
        array_83.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_86) = &input.resource_arn {
        object.key("ResourceArn").string(var_86);
    }
    if let Some(var_87) = &input.tag_keys {
        let mut array_88 = object.key("TagKeys").start_array();
        for item_89 in var_87 {
            {
                array_88.value().string(item_89);
            }
        }
        array_88.finish();
    }
}

pub fn serialize_structure_update_inference_scheduler_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateInferenceSchedulerInput,
) {
    if let Some(var_90) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_90);
    }
    if let Some(var_91) = &input.data_delay_offset_in_minutes {
        object.key("DataDelayOffsetInMinutes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_91).into()),
        );
    }
    if let Some(var_92) = &input.data_upload_frequency {
        object.key("DataUploadFrequency").string(var_92.as_str());
    }
    if let Some(var_93) = &input.data_input_configuration {
        let mut object_94 = object.key("DataInputConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_input_configuration(&mut object_94, var_93);
        object_94.finish();
    }
    if let Some(var_95) = &input.data_output_configuration {
        let mut object_96 = object.key("DataOutputConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_output_configuration(&mut object_96, var_95);
        object_96.finish();
    }
    if let Some(var_97) = &input.role_arn {
        object.key("RoleArn").string(var_97);
    }
}

pub fn serialize_structure_dataset_schema(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetSchema,
) {
    if let Some(var_98) = &input.inline_data_schema {
        object.key("InlineDataSchema").string(var_98);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_99) = &input.key {
        object.key("Key").string(var_99);
    }
    if let Some(var_100) = &input.value {
        object.key("Value").string(var_100);
    }
}

pub fn serialize_structure_inference_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceInputConfiguration,
) {
    if let Some(var_101) = &input.s3_input_configuration {
        let mut object_102 = object.key("S3InputConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_s3_input_configuration(
            &mut object_102,
            var_101,
        );
        object_102.finish();
    }
    if let Some(var_103) = &input.input_time_zone_offset {
        object.key("InputTimeZoneOffset").string(var_103);
    }
    if let Some(var_104) = &input.inference_input_name_configuration {
        let mut object_105 = object.key("InferenceInputNameConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_input_name_configuration(
            &mut object_105,
            var_104,
        );
        object_105.finish();
    }
}

pub fn serialize_structure_inference_output_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceOutputConfiguration,
) {
    if let Some(var_106) = &input.s3_output_configuration {
        let mut object_107 = object.key("S3OutputConfiguration").start_object();
        crate::json_ser::serialize_structure_inference_s3_output_configuration(
            &mut object_107,
            var_106,
        );
        object_107.finish();
    }
    if let Some(var_108) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_108);
    }
}

pub fn serialize_structure_labels_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LabelsInputConfiguration,
) {
    if let Some(var_109) = &input.s3_input_configuration {
        let mut object_110 = object.key("S3InputConfiguration").start_object();
        crate::json_ser::serialize_structure_labels_s3_input_configuration(
            &mut object_110,
            var_109,
        );
        object_110.finish();
    }
}

pub fn serialize_structure_data_pre_processing_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataPreProcessingConfiguration,
) {
    if let Some(var_111) = &input.target_sampling_rate {
        object.key("TargetSamplingRate").string(var_111.as_str());
    }
}

pub fn serialize_structure_ingestion_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IngestionInputConfiguration,
) {
    if let Some(var_112) = &input.s3_input_configuration {
        let mut object_113 = object.key("S3InputConfiguration").start_object();
        crate::json_ser::serialize_structure_ingestion_s3_input_configuration(
            &mut object_113,
            var_112,
        );
        object_113.finish();
    }
}

pub fn serialize_structure_inference_s3_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceS3InputConfiguration,
) {
    if let Some(var_114) = &input.bucket {
        object.key("Bucket").string(var_114);
    }
    if let Some(var_115) = &input.prefix {
        object.key("Prefix").string(var_115);
    }
}

pub fn serialize_structure_inference_input_name_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceInputNameConfiguration,
) {
    if let Some(var_116) = &input.timestamp_format {
        object.key("TimestampFormat").string(var_116);
    }
    if let Some(var_117) = &input.component_timestamp_delimiter {
        object.key("ComponentTimestampDelimiter").string(var_117);
    }
}

pub fn serialize_structure_inference_s3_output_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceS3OutputConfiguration,
) {
    if let Some(var_118) = &input.bucket {
        object.key("Bucket").string(var_118);
    }
    if let Some(var_119) = &input.prefix {
        object.key("Prefix").string(var_119);
    }
}

pub fn serialize_structure_labels_s3_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LabelsS3InputConfiguration,
) {
    if let Some(var_120) = &input.bucket {
        object.key("Bucket").string(var_120);
    }
    if let Some(var_121) = &input.prefix {
        object.key("Prefix").string(var_121);
    }
}

pub fn serialize_structure_ingestion_s3_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IngestionS3InputConfiguration,
) {
    if let Some(var_122) = &input.bucket {
        object.key("Bucket").string(var_122);
    }
    if let Some(var_123) = &input.prefix {
        object.key("Prefix").string(var_123);
    }
}