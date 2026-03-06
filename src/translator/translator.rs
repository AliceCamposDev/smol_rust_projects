use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

pub fn translator_fn(){
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Portuguese])
        .create_model().unwrap();

    let input = ["In findings published Tuesday in Cornell University's arXiv by a team of scientists
from the University of Montreal and a separate report published Wednesday in Nature Astronomy by a team
from University College London (UCL), the presence of water vapour was confirmed in the atmosphere of K2-18b"];

    let output = model.translate(&input, Language::English, Language::Portuguese).unwrap();

    println!("Translation: {}", output[0]);
}