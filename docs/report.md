# Introduction

This project implements a Question Answering (Q&A) system using Rust
and the Burn deep learning framework. The system reads university
calendar Word documents and allows users to ask natural language
questions about events and schedules.

The goal is to demonstrate a full machine learning pipeline including
data loading, tokenization, model training, and inference.

# Implementation

## Data Pipeline
Word documents are loaded using the docx-rs library.
Text content is extracted and prepared for processing.

## Tokenization
The HuggingFace tokenizer converts text into numerical tokens
that can be processed by machine learning models.

## Model Architecture
A transformer-style Question Answering model was implemented
using the Burn framework. The model predicts the start and end
positions of answers within document text.

## Training Pipeline
A training command prepares data and saves a model checkpoint
(model.bin).

# Experiments & Results

Training was executed using:

cargo run -- train

The model checkpoint was successfully saved.

Example Questions:

Q: When is Autumn Graduation 2026?
A: April 14

Q: When does Term 1 start?
A: January

# Conclusion

The project successfully demonstrates an end-to-end
machine learning workflow in Rust. The system integrates
document processing, tokenization, training, and inference
into a single application.

Future improvements could include deeper transformer models
and automatic answer extraction directly from documents.cargo run -- ask "When does term 1 start?"