use aws_lambda_events::sqs::{SqsBatchResponse, SqsEvent};
use aws_sdk_sqs::operation::send_message::builders::SendMessageFluentBuilder;
use kinetics_macro::worker;
use lambda_http::Error;
use lambda_runtime::LambdaEvent;
use std::collections::HashMap;

#[worker(concurrency = 3, fifo = true, queue_alias = "example")]
pub async fn worker(
    _event: LambdaEvent<SqsEvent>,
    _secrets: &HashMap<String, String>,
    _queues: &HashMap<String, SendMessageFluentBuilder>,
) -> Result<SqsBatchResponse, Error> {
    let sqs_batch_response = SqsBatchResponse::default();
    Ok(sqs_batch_response)
}
