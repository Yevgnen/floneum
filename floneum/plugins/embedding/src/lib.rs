use floneum_rust::*;

#[export_plugin]
/// Creates embeddings for some text.
///
/// An embedding is a representation of something like the "meaning" of some text. You can use embeddings with embedding databases to find documents similar to anther document.
///
/// ### Examples
/// vec![
///     Example {
///         name: "example".into(),
///         inputs: vec![ModelType::Llama(LlamaType::LlamaSevenChat).into_input_value(), String::from("Text to embed").into_input_value()],
///         outputs: vec![Embedding { vector: vec![0.0, 0.0, 0.0] }.into_return_value()],
///     },
/// ]
fn embedding(model_type: ModelType, input: String) -> Embedding {
    let model = ModelInstance::new(model_type);

    model.get_embedding(&input)
}
