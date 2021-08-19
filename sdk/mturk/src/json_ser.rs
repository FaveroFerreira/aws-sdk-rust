// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_accept_qualification_request_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptQualificationRequestInput,
) {
    if let Some(var_1) = &input.qualification_request_id {
        object.key("QualificationRequestId").string(var_1);
    }
    if let Some(var_2) = &input.integer_value {
        object.key("IntegerValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_2).into()),
        );
    }
}

pub fn serialize_structure_approve_assignment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ApproveAssignmentInput,
) {
    if let Some(var_3) = &input.assignment_id {
        object.key("AssignmentId").string(var_3);
    }
    if let Some(var_4) = &input.requester_feedback {
        object.key("RequesterFeedback").string(var_4);
    }
    if let Some(var_5) = &input.override_rejection {
        object.key("OverrideRejection").boolean(*var_5);
    }
}

pub fn serialize_structure_associate_qualification_with_worker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateQualificationWithWorkerInput,
) {
    if let Some(var_6) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_6);
    }
    if let Some(var_7) = &input.worker_id {
        object.key("WorkerId").string(var_7);
    }
    if let Some(var_8) = &input.integer_value {
        object.key("IntegerValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.send_notification {
        object.key("SendNotification").boolean(*var_9);
    }
}

pub fn serialize_structure_create_additional_assignments_for_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAdditionalAssignmentsForHitInput,
) {
    if let Some(var_10) = &input.hit_id {
        object.key("HITId").string(var_10);
    }
    if let Some(var_11) = &input.number_of_additional_assignments {
        object.key("NumberOfAdditionalAssignments").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.unique_request_token {
        object.key("UniqueRequestToken").string(var_12);
    }
}

pub fn serialize_structure_create_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHitInput,
) {
    if let Some(var_13) = &input.max_assignments {
        object.key("MaxAssignments").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.auto_approval_delay_in_seconds {
        object.key("AutoApprovalDelayInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.lifetime_in_seconds {
        object.key("LifetimeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    if let Some(var_16) = &input.assignment_duration_in_seconds {
        object.key("AssignmentDurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.reward {
        object.key("Reward").string(var_17);
    }
    if let Some(var_18) = &input.title {
        object.key("Title").string(var_18);
    }
    if let Some(var_19) = &input.keywords {
        object.key("Keywords").string(var_19);
    }
    if let Some(var_20) = &input.description {
        object.key("Description").string(var_20);
    }
    if let Some(var_21) = &input.question {
        object.key("Question").string(var_21);
    }
    if let Some(var_22) = &input.requester_annotation {
        object.key("RequesterAnnotation").string(var_22);
    }
    if let Some(var_23) = &input.qualification_requirements {
        let mut array_24 = object.key("QualificationRequirements").start_array();
        for item_25 in var_23 {
            {
                let mut object_26 = array_24.value().start_object();
                crate::json_ser::serialize_structure_qualification_requirement(
                    &mut object_26,
                    item_25,
                );
                object_26.finish();
            }
        }
        array_24.finish();
    }
    if let Some(var_27) = &input.unique_request_token {
        object.key("UniqueRequestToken").string(var_27);
    }
    if let Some(var_28) = &input.assignment_review_policy {
        let mut object_29 = object.key("AssignmentReviewPolicy").start_object();
        crate::json_ser::serialize_structure_review_policy(&mut object_29, var_28);
        object_29.finish();
    }
    if let Some(var_30) = &input.hit_review_policy {
        let mut object_31 = object.key("HITReviewPolicy").start_object();
        crate::json_ser::serialize_structure_review_policy(&mut object_31, var_30);
        object_31.finish();
    }
    if let Some(var_32) = &input.hit_layout_id {
        object.key("HITLayoutId").string(var_32);
    }
    if let Some(var_33) = &input.hit_layout_parameters {
        let mut array_34 = object.key("HITLayoutParameters").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_hit_layout_parameter(&mut object_36, item_35);
                object_36.finish();
            }
        }
        array_34.finish();
    }
}

pub fn serialize_structure_create_hit_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHitTypeInput,
) {
    if let Some(var_37) = &input.auto_approval_delay_in_seconds {
        object.key("AutoApprovalDelayInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.assignment_duration_in_seconds {
        object.key("AssignmentDurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_38).into()),
        );
    }
    if let Some(var_39) = &input.reward {
        object.key("Reward").string(var_39);
    }
    if let Some(var_40) = &input.title {
        object.key("Title").string(var_40);
    }
    if let Some(var_41) = &input.keywords {
        object.key("Keywords").string(var_41);
    }
    if let Some(var_42) = &input.description {
        object.key("Description").string(var_42);
    }
    if let Some(var_43) = &input.qualification_requirements {
        let mut array_44 = object.key("QualificationRequirements").start_array();
        for item_45 in var_43 {
            {
                let mut object_46 = array_44.value().start_object();
                crate::json_ser::serialize_structure_qualification_requirement(
                    &mut object_46,
                    item_45,
                );
                object_46.finish();
            }
        }
        array_44.finish();
    }
}

pub fn serialize_structure_create_hit_with_hit_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHitWithHitTypeInput,
) {
    if let Some(var_47) = &input.hit_type_id {
        object.key("HITTypeId").string(var_47);
    }
    if let Some(var_48) = &input.max_assignments {
        object.key("MaxAssignments").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_48).into()),
        );
    }
    if let Some(var_49) = &input.lifetime_in_seconds {
        object.key("LifetimeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    if let Some(var_50) = &input.question {
        object.key("Question").string(var_50);
    }
    if let Some(var_51) = &input.requester_annotation {
        object.key("RequesterAnnotation").string(var_51);
    }
    if let Some(var_52) = &input.unique_request_token {
        object.key("UniqueRequestToken").string(var_52);
    }
    if let Some(var_53) = &input.assignment_review_policy {
        let mut object_54 = object.key("AssignmentReviewPolicy").start_object();
        crate::json_ser::serialize_structure_review_policy(&mut object_54, var_53);
        object_54.finish();
    }
    if let Some(var_55) = &input.hit_review_policy {
        let mut object_56 = object.key("HITReviewPolicy").start_object();
        crate::json_ser::serialize_structure_review_policy(&mut object_56, var_55);
        object_56.finish();
    }
    if let Some(var_57) = &input.hit_layout_id {
        object.key("HITLayoutId").string(var_57);
    }
    if let Some(var_58) = &input.hit_layout_parameters {
        let mut array_59 = object.key("HITLayoutParameters").start_array();
        for item_60 in var_58 {
            {
                let mut object_61 = array_59.value().start_object();
                crate::json_ser::serialize_structure_hit_layout_parameter(&mut object_61, item_60);
                object_61.finish();
            }
        }
        array_59.finish();
    }
}

pub fn serialize_structure_create_qualification_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateQualificationTypeInput,
) {
    if let Some(var_62) = &input.name {
        object.key("Name").string(var_62);
    }
    if let Some(var_63) = &input.keywords {
        object.key("Keywords").string(var_63);
    }
    if let Some(var_64) = &input.description {
        object.key("Description").string(var_64);
    }
    if let Some(var_65) = &input.qualification_type_status {
        object
            .key("QualificationTypeStatus")
            .string(var_65.as_str());
    }
    if let Some(var_66) = &input.retry_delay_in_seconds {
        object.key("RetryDelayInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_66).into()),
        );
    }
    if let Some(var_67) = &input.test {
        object.key("Test").string(var_67);
    }
    if let Some(var_68) = &input.answer_key {
        object.key("AnswerKey").string(var_68);
    }
    if let Some(var_69) = &input.test_duration_in_seconds {
        object.key("TestDurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_69).into()),
        );
    }
    if let Some(var_70) = &input.auto_granted {
        object.key("AutoGranted").boolean(*var_70);
    }
    if let Some(var_71) = &input.auto_granted_value {
        object.key("AutoGrantedValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_71).into()),
        );
    }
}

pub fn serialize_structure_create_worker_block_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkerBlockInput,
) {
    if let Some(var_72) = &input.worker_id {
        object.key("WorkerId").string(var_72);
    }
    if let Some(var_73) = &input.reason {
        object.key("Reason").string(var_73);
    }
}

pub fn serialize_structure_delete_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteHitInput,
) {
    if let Some(var_74) = &input.hit_id {
        object.key("HITId").string(var_74);
    }
}

pub fn serialize_structure_delete_qualification_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteQualificationTypeInput,
) {
    if let Some(var_75) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_75);
    }
}

pub fn serialize_structure_delete_worker_block_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteWorkerBlockInput,
) {
    if let Some(var_76) = &input.worker_id {
        object.key("WorkerId").string(var_76);
    }
    if let Some(var_77) = &input.reason {
        object.key("Reason").string(var_77);
    }
}

pub fn serialize_structure_disassociate_qualification_from_worker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateQualificationFromWorkerInput,
) {
    if let Some(var_78) = &input.worker_id {
        object.key("WorkerId").string(var_78);
    }
    if let Some(var_79) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_79);
    }
    if let Some(var_80) = &input.reason {
        object.key("Reason").string(var_80);
    }
}

pub fn serialize_structure_get_assignment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAssignmentInput,
) {
    if let Some(var_81) = &input.assignment_id {
        object.key("AssignmentId").string(var_81);
    }
}

pub fn serialize_structure_get_file_upload_url_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetFileUploadUrlInput,
) {
    if let Some(var_82) = &input.assignment_id {
        object.key("AssignmentId").string(var_82);
    }
    if let Some(var_83) = &input.question_identifier {
        object.key("QuestionIdentifier").string(var_83);
    }
}

pub fn serialize_structure_get_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetHitInput,
) {
    if let Some(var_84) = &input.hit_id {
        object.key("HITId").string(var_84);
    }
}

pub fn serialize_structure_get_qualification_score_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQualificationScoreInput,
) {
    if let Some(var_85) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_85);
    }
    if let Some(var_86) = &input.worker_id {
        object.key("WorkerId").string(var_86);
    }
}

pub fn serialize_structure_get_qualification_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQualificationTypeInput,
) {
    if let Some(var_87) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_87);
    }
}

pub fn serialize_structure_list_assignments_for_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAssignmentsForHitInput,
) {
    if let Some(var_88) = &input.hit_id {
        object.key("HITId").string(var_88);
    }
    if let Some(var_89) = &input.next_token {
        object.key("NextToken").string(var_89);
    }
    if let Some(var_90) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_90).into()),
        );
    }
    if let Some(var_91) = &input.assignment_statuses {
        let mut array_92 = object.key("AssignmentStatuses").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
}

pub fn serialize_structure_list_bonus_payments_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListBonusPaymentsInput,
) {
    if let Some(var_94) = &input.hit_id {
        object.key("HITId").string(var_94);
    }
    if let Some(var_95) = &input.assignment_id {
        object.key("AssignmentId").string(var_95);
    }
    if let Some(var_96) = &input.next_token {
        object.key("NextToken").string(var_96);
    }
    if let Some(var_97) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_97).into()),
        );
    }
}

pub fn serialize_structure_list_hi_ts_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListHiTsInput,
) {
    if let Some(var_98) = &input.next_token {
        object.key("NextToken").string(var_98);
    }
    if let Some(var_99) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_99).into()),
        );
    }
}

pub fn serialize_structure_list_hi_ts_for_qualification_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListHiTsForQualificationTypeInput,
) {
    if let Some(var_100) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_100);
    }
    if let Some(var_101) = &input.next_token {
        object.key("NextToken").string(var_101);
    }
    if let Some(var_102) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_102).into()),
        );
    }
}

pub fn serialize_structure_list_qualification_requests_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListQualificationRequestsInput,
) {
    if let Some(var_103) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_103);
    }
    if let Some(var_104) = &input.next_token {
        object.key("NextToken").string(var_104);
    }
    if let Some(var_105) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_105).into()),
        );
    }
}

pub fn serialize_structure_list_qualification_types_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListQualificationTypesInput,
) {
    if let Some(var_106) = &input.query {
        object.key("Query").string(var_106);
    }
    if let Some(var_107) = &input.must_be_requestable {
        object.key("MustBeRequestable").boolean(*var_107);
    }
    if let Some(var_108) = &input.must_be_owned_by_caller {
        object.key("MustBeOwnedByCaller").boolean(*var_108);
    }
    if let Some(var_109) = &input.next_token {
        object.key("NextToken").string(var_109);
    }
    if let Some(var_110) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_110).into()),
        );
    }
}

pub fn serialize_structure_list_reviewable_hi_ts_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListReviewableHiTsInput,
) {
    if let Some(var_111) = &input.hit_type_id {
        object.key("HITTypeId").string(var_111);
    }
    if let Some(var_112) = &input.status {
        object.key("Status").string(var_112.as_str());
    }
    if let Some(var_113) = &input.next_token {
        object.key("NextToken").string(var_113);
    }
    if let Some(var_114) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_114).into()),
        );
    }
}

pub fn serialize_structure_list_review_policy_results_for_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListReviewPolicyResultsForHitInput,
) {
    if let Some(var_115) = &input.hit_id {
        object.key("HITId").string(var_115);
    }
    if let Some(var_116) = &input.policy_levels {
        let mut array_117 = object.key("PolicyLevels").start_array();
        for item_118 in var_116 {
            {
                array_117.value().string(item_118.as_str());
            }
        }
        array_117.finish();
    }
    if let Some(var_119) = &input.retrieve_actions {
        object.key("RetrieveActions").boolean(*var_119);
    }
    if let Some(var_120) = &input.retrieve_results {
        object.key("RetrieveResults").boolean(*var_120);
    }
    if let Some(var_121) = &input.next_token {
        object.key("NextToken").string(var_121);
    }
    if let Some(var_122) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_122).into()),
        );
    }
}

pub fn serialize_structure_list_worker_blocks_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListWorkerBlocksInput,
) {
    if let Some(var_123) = &input.next_token {
        object.key("NextToken").string(var_123);
    }
    if let Some(var_124) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_124).into()),
        );
    }
}

pub fn serialize_structure_list_workers_with_qualification_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListWorkersWithQualificationTypeInput,
) {
    if let Some(var_125) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_125);
    }
    if let Some(var_126) = &input.status {
        object.key("Status").string(var_126.as_str());
    }
    if let Some(var_127) = &input.next_token {
        object.key("NextToken").string(var_127);
    }
    if let Some(var_128) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_128).into()),
        );
    }
}

pub fn serialize_structure_notify_workers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::NotifyWorkersInput,
) {
    if let Some(var_129) = &input.subject {
        object.key("Subject").string(var_129);
    }
    if let Some(var_130) = &input.message_text {
        object.key("MessageText").string(var_130);
    }
    if let Some(var_131) = &input.worker_ids {
        let mut array_132 = object.key("WorkerIds").start_array();
        for item_133 in var_131 {
            {
                array_132.value().string(item_133);
            }
        }
        array_132.finish();
    }
}

pub fn serialize_structure_reject_assignment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectAssignmentInput,
) {
    if let Some(var_134) = &input.assignment_id {
        object.key("AssignmentId").string(var_134);
    }
    if let Some(var_135) = &input.requester_feedback {
        object.key("RequesterFeedback").string(var_135);
    }
}

pub fn serialize_structure_reject_qualification_request_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectQualificationRequestInput,
) {
    if let Some(var_136) = &input.qualification_request_id {
        object.key("QualificationRequestId").string(var_136);
    }
    if let Some(var_137) = &input.reason {
        object.key("Reason").string(var_137);
    }
}

pub fn serialize_structure_send_bonus_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendBonusInput,
) {
    if let Some(var_138) = &input.worker_id {
        object.key("WorkerId").string(var_138);
    }
    if let Some(var_139) = &input.bonus_amount {
        object.key("BonusAmount").string(var_139);
    }
    if let Some(var_140) = &input.assignment_id {
        object.key("AssignmentId").string(var_140);
    }
    if let Some(var_141) = &input.reason {
        object.key("Reason").string(var_141);
    }
    if let Some(var_142) = &input.unique_request_token {
        object.key("UniqueRequestToken").string(var_142);
    }
}

pub fn serialize_structure_send_test_event_notification_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendTestEventNotificationInput,
) {
    if let Some(var_143) = &input.notification {
        let mut object_144 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_notification_specification(&mut object_144, var_143);
        object_144.finish();
    }
    if let Some(var_145) = &input.test_event_type {
        object.key("TestEventType").string(var_145.as_str());
    }
}

pub fn serialize_structure_update_expiration_for_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateExpirationForHitInput,
) {
    if let Some(var_146) = &input.hit_id {
        object.key("HITId").string(var_146);
    }
    if let Some(var_147) = &input.expire_at {
        object
            .key("ExpireAt")
            .instant(var_147, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_update_hit_review_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateHitReviewStatusInput,
) {
    if let Some(var_148) = &input.hit_id {
        object.key("HITId").string(var_148);
    }
    if let Some(var_149) = &input.revert {
        object.key("Revert").boolean(*var_149);
    }
}

pub fn serialize_structure_update_hit_type_of_hit_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateHitTypeOfHitInput,
) {
    if let Some(var_150) = &input.hit_id {
        object.key("HITId").string(var_150);
    }
    if let Some(var_151) = &input.hit_type_id {
        object.key("HITTypeId").string(var_151);
    }
}

pub fn serialize_structure_update_notification_settings_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNotificationSettingsInput,
) {
    if let Some(var_152) = &input.hit_type_id {
        object.key("HITTypeId").string(var_152);
    }
    if let Some(var_153) = &input.notification {
        let mut object_154 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_notification_specification(&mut object_154, var_153);
        object_154.finish();
    }
    if let Some(var_155) = &input.active {
        object.key("Active").boolean(*var_155);
    }
}

pub fn serialize_structure_update_qualification_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateQualificationTypeInput,
) {
    if let Some(var_156) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_156);
    }
    if let Some(var_157) = &input.description {
        object.key("Description").string(var_157);
    }
    if let Some(var_158) = &input.qualification_type_status {
        object
            .key("QualificationTypeStatus")
            .string(var_158.as_str());
    }
    if let Some(var_159) = &input.test {
        object.key("Test").string(var_159);
    }
    if let Some(var_160) = &input.answer_key {
        object.key("AnswerKey").string(var_160);
    }
    if let Some(var_161) = &input.test_duration_in_seconds {
        object.key("TestDurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_161).into()),
        );
    }
    if let Some(var_162) = &input.retry_delay_in_seconds {
        object.key("RetryDelayInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_162).into()),
        );
    }
    if let Some(var_163) = &input.auto_granted {
        object.key("AutoGranted").boolean(*var_163);
    }
    if let Some(var_164) = &input.auto_granted_value {
        object.key("AutoGrantedValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_164).into()),
        );
    }
}

pub fn serialize_structure_qualification_requirement(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::QualificationRequirement,
) {
    if let Some(var_165) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_165);
    }
    if let Some(var_166) = &input.comparator {
        object.key("Comparator").string(var_166.as_str());
    }
    if let Some(var_167) = &input.integer_values {
        let mut array_168 = object.key("IntegerValues").start_array();
        for item_169 in var_167 {
            {
                array_168.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::NegInt((*item_169).into()),
                );
            }
        }
        array_168.finish();
    }
    if let Some(var_170) = &input.locale_values {
        let mut array_171 = object.key("LocaleValues").start_array();
        for item_172 in var_170 {
            {
                let mut object_173 = array_171.value().start_object();
                crate::json_ser::serialize_structure_locale(&mut object_173, item_172);
                object_173.finish();
            }
        }
        array_171.finish();
    }
    if let Some(var_174) = &input.required_to_preview {
        object.key("RequiredToPreview").boolean(*var_174);
    }
    if let Some(var_175) = &input.actions_guarded {
        object.key("ActionsGuarded").string(var_175.as_str());
    }
}

pub fn serialize_structure_review_policy(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReviewPolicy,
) {
    if let Some(var_176) = &input.policy_name {
        object.key("PolicyName").string(var_176);
    }
    if let Some(var_177) = &input.parameters {
        let mut array_178 = object.key("Parameters").start_array();
        for item_179 in var_177 {
            {
                let mut object_180 = array_178.value().start_object();
                crate::json_ser::serialize_structure_policy_parameter(&mut object_180, item_179);
                object_180.finish();
            }
        }
        array_178.finish();
    }
}

pub fn serialize_structure_hit_layout_parameter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HitLayoutParameter,
) {
    if let Some(var_181) = &input.name {
        object.key("Name").string(var_181);
    }
    if let Some(var_182) = &input.value {
        object.key("Value").string(var_182);
    }
}

pub fn serialize_structure_notification_specification(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationSpecification,
) {
    if let Some(var_183) = &input.destination {
        object.key("Destination").string(var_183);
    }
    if let Some(var_184) = &input.transport {
        object.key("Transport").string(var_184.as_str());
    }
    if let Some(var_185) = &input.version {
        object.key("Version").string(var_185);
    }
    if let Some(var_186) = &input.event_types {
        let mut array_187 = object.key("EventTypes").start_array();
        for item_188 in var_186 {
            {
                array_187.value().string(item_188.as_str());
            }
        }
        array_187.finish();
    }
}

pub fn serialize_structure_locale(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Locale,
) {
    if let Some(var_189) = &input.country {
        object.key("Country").string(var_189);
    }
    if let Some(var_190) = &input.subdivision {
        object.key("Subdivision").string(var_190);
    }
}

pub fn serialize_structure_policy_parameter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyParameter,
) {
    if let Some(var_191) = &input.key {
        object.key("Key").string(var_191);
    }
    if let Some(var_192) = &input.values {
        let mut array_193 = object.key("Values").start_array();
        for item_194 in var_192 {
            {
                array_193.value().string(item_194);
            }
        }
        array_193.finish();
    }
    if let Some(var_195) = &input.map_entries {
        let mut array_196 = object.key("MapEntries").start_array();
        for item_197 in var_195 {
            {
                let mut object_198 = array_196.value().start_object();
                crate::json_ser::serialize_structure_parameter_map_entry(&mut object_198, item_197);
                object_198.finish();
            }
        }
        array_196.finish();
    }
}

pub fn serialize_structure_parameter_map_entry(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParameterMapEntry,
) {
    if let Some(var_199) = &input.key {
        object.key("Key").string(var_199);
    }
    if let Some(var_200) = &input.values {
        let mut array_201 = object.key("Values").start_array();
        for item_202 in var_200 {
            {
                array_201.value().string(item_202);
            }
        }
        array_201.finish();
    }
}