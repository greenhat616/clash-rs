#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Addons {
    #[prost(string, tag = "1")]
    pub flow: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub seed: ::prost::alloc::vec::Vec<u8>,
}
