pub mod macros;
pub mod properties;
pub mod util;

mod branch_protection_rule;
mod create;
mod delete;
mod events;
mod label;
mod meta;
mod ping;
mod pull_request;
mod pull_request_review;
mod push;
mod release;
mod repository;
mod star;
mod status;

pub use branch_protection_rule::*;
pub use create::*;
pub use delete::*;
pub use events::*;
pub use label::*;
pub use meta::*;
pub use ping::*;
pub use pull_request::*;
pub use pull_request_review::*;
pub use push::*;
pub use release::*;
pub use repository::*;
pub use star::*;
pub use status::*;

impl_events_unwrapper! {
  #[derive(Debug)]
  pub enum Events {
    BranchProtectionRule(BranchProtectionRuleEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    Label(LabelEvent),
    Meta(MetaEvent),
    Ping(PingEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    Push(PushEvent),
    Release(ReleaseEvent),
    Repository(RepositoryEvent),
    Star(StarEvent),
    Status(StatusEvent),
  }
}
