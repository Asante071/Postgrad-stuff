use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

#[derive(Module, Debug)]
pub struct QAModel<B: Backend> {
    linear_start: Linear<B>,
    linear_end: Linear<B>,
}

impl<B: Backend> QAModel<B> {
    pub fn new(hidden: usize) -> Self {
        Self {
            linear_start: LinearConfig::new(hidden, 1).init(),
            linear_end: LinearConfig::new(hidden, 1).init(),
        }
    }

    pub fn forward(&self, x: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let start_logits = self.linear_start.forward(x.clone());
        let end_logits = self.linear_end.forward(x);

        (start_logits, end_logits)
    }
}