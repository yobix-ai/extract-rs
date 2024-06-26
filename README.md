# Extract-RS

Extract-RS offers a unified approach for detecting and extracting metadata and text content from various documents
types such as PDF, Word, HTML, and [many other formats](#supported-file-formats).
Our goal is to deliver an efficient comprehensive solution with bindings for many programming languages.

## Why Extract-RS?

Extract-RS was mainly inspired by the [Unstructured Python library](https://github.com/Unstructured-IO/unstructured).
While Unstructured offers a good solution for parsing unstructured content, we see 2 main issues with it:

* Performance: data processing is mainly a cpu-bound problem and Python is not the best choice for such tasks
  because of its Global Interpreter Lock (GIL) which makes it hard to utilize multiple cores.
* [Unstructured](https://github.com/Unstructured-IO/unstructured) is becoming more of an LLM framework rather than
  just text and metadata parsing library.

Extract-RS will focus only on the text and metadata extraction part. The core is written in Rust, leveraging its
memory safety, multithreading and zero cost abstractions. Extract-RS will provide bindings for many programming
languages.

## Features

* Clear simple API for extracting text and metadata content.
* Support for [many file formats](#supported-file-formats).
* Strives to be efficient and fast.
* Comprehensive documentation and examples to help you get started quickly.

## Bindings

| Name                                        | Release                                                                                |
|---------------------------------------------|----------------------------------------------------------------------------------------|
| [Rust Core](extract-rs/README.md)           | [![](https://img.shields.io/crates/v/extract-rs)](https://crates.io/crates/extract-rs) |
| [Python Binding](bindings/python/README.md) | [![](https://img.shields.io/pypi/v/extract-rs)](https://pypi.org/project/extract-rs/)  |

## Supported file formats

| File Format | Rust Core | Python Binding |
|-------------|-----------|----------------|
| pdf         | ✅         | ✅              |
| csv         | ✅         | ✅              |
