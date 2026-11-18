#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Session(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FilePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Tokens(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Response(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WorkId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum WorkKind {
    Shell,
    Subagent,
    Monitor,
    Workflow,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Description(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BackgroundWork {
    pub work_id: WorkId,
    pub work_kind: WorkKind,
    pub description: Description,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Trigger {
    Manual,
    Automatic,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Compaction {
    pub trigger: Trigger,
    pub tokens_option: std::option::Option<Tokens>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Failure(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EndReason {
    Exited,
    Cleared,
    Died,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Event {
    Started,
    PromptSubmitted,
    ToolUsed,
    PermissionAsked,
    Responded(Response),
    InFlight(std::vec::Vec<BackgroundWork>),
    WorkStarted(BackgroundWork),
    WorkEnded(WorkId),
    TurnEnded,
    Interrupted,
    Failed(Failure),
    ContextSize(Tokens),
    Compacted(Compaction),
    Ended(EndReason),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum State {
    Dry,
    Flowing(Flow),
    Ended,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Flow {
    Working,
    Rested,
    Waiting,
    Blocked,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Observed {
    pub flow_id: flow::FlowId,
    pub state: State,
    pub tokens: Tokens,
    pub background_work_vector: std::vec::Vec<BackgroundWork>,
}
#[rustfmt::skip]
pub trait Observable {
    fn events(&self) -> std::vec::Vec<Event>;
}
#[rustfmt::skip]
pub trait Transitionable {
    fn after(&mut self, input: Event);
}
#[rustfmt::skip]
const _: () = {
    fn assert_observed_transitionable<T: Transitionable>() {}
    let _ = assert_observed_transitionable::<Observed>;
};
