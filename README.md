# blocktest [![Build Status](https://travis-ci.org/jdkato/blocktest.svg?branch=master)](https://travis-ci.org/jdkato/blocktest)

`blocktest` is a "preprocessor" for markup (e.g., Markdown or reStructuredText) code blocks. It allows you to keep your code and prose separate without duplicating work. The idea is simple: `blocktest` extracts your code examples from within a larger testing context and adds them to your markup at user-specified locations.

For instance, let's say that you're writing this [section of the spaCy documentation](https://spacy.io/api/language#call). In this example, you want to include the following Python code snippet in your explanation:

```python
doc = nlp(u'An example sentence. Another sentence.')
assert (doc[0].text, doc[0].head.tag_) == ('An', 'NN')
```

With `blocktest`, you need two files:

1. A Python test file with the snippet inside of its test-related boilerplate:

    ```python
    # test_language.py
    import spacy

    nlp = spacy.load('en')

    def test_example():
        doc = nlp(u'An example sentence. Another sentence.')  # example1 begin
        assert (doc[0].text, doc[0].head.tag_) == ('An', 'NN')  # example1 end
     ```
 2. A markup file which specifies where to find the snippet:
 
     ````
     <!-- example.md -->

     ```python
     {{< id="example1" src="test_language.py" >}}
     ```
     ````
     
  `blocktest` will scan `example.md` looking for *block definitions*, which specify a unique ID and a source file for each snippet. It will then extract the snippet (as indicated by the `<id> begin` / `<id> end` comments) from the source file and substitute it back into your markup. This allows you to write a thorough test suite for your code examples without having to maintain multiple copies of the same snippet.

## Getting Started

