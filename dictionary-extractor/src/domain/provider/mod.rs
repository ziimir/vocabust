pub mod dict_provider;
pub mod oxford_dict_provider;
pub mod translation_provider;

pub use dict_provider::DictProviderError;
pub use oxford_dict_provider::OxfordDictProvider;
pub use translation_provider::translate_word_data;
