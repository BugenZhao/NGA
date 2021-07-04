use crate::{
    error::LogicResult,
    protos::Service::{
        PostVoteRequest, PostVoteRequest_Operation, PostVoteResponse, PostVoteResponse_oneof__error,
    },
    service::utils::extract_string,
};

use super::fetch_package;

pub async fn post_vote(request: PostVoteRequest) -> LogicResult<PostVoteResponse> {
    let value = match request.get_operation() {
        PostVoteRequest_Operation::UPVOTE => "1",
        PostVoteRequest_Operation::DOWNVOTE => "-1",
    };

    let package = fetch_package(
        "nuke.php",
        vec![
            ("__lib", "topic_recommend"),
            ("__act", "add"),
            ("value", value),
            ("tid", request.get_post_id().get_tid()),
            ("pid", request.get_post_id().get_pid()),
        ],
        vec![],
    )
    .await?;

    if let Ok(delta) = extract_string(&package, "/root/data/item[2]") {
        let delta = delta.parse::<i32>().unwrap_or_default();
        Ok(PostVoteResponse {
            delta,
            ..Default::default()
        })
    } else {
        let error = extract_string(&package, "/root/error/item[1]").unwrap_or_default();
        Ok(PostVoteResponse {
            _error: Some(error).map(PostVoteResponse_oneof__error::error),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::protos::DataModel::PostId;

    #[tokio::test]
    async fn test_post_vote() -> LogicResult<()> {
        use PostVoteRequest_Operation::*;
        let vote = |op| {
            post_vote(PostVoteRequest {
                post_id: Some(PostId {
                    tid: "27375475".to_owned(),
                    pid: "0".to_owned(),
                    ..Default::default()
                })
                .into(),
                operation: op,
                ..Default::default()
            })
        };

        while vote(UPVOTE).await.unwrap().delta != -1 {}

        assert_eq!(vote(UPVOTE).await.unwrap().delta, 1);
        assert_eq!(vote(UPVOTE).await.unwrap().delta, -1);
        assert_eq!(vote(UPVOTE).await.unwrap().delta, 1);
        assert_eq!(vote(DOWNVOTE).await.unwrap().delta, -2);
        assert_eq!(vote(DOWNVOTE).await.unwrap().delta, 1);

        Ok(())
    }
}
