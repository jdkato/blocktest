Custom components can be added to the pipeline using the `add_pipe`  method. Optionally, you can either specify a component to add it before or after, tell spaCy to add it first or last in the pipeline, or define a custom name. If no name is set and no name attribute is present on your component, the function name is used.

```python
def my_component(doc):  
    print("After tokenization, this doc has %s tokens." % len(doc))
    if len(doc) < 10:
        print("This is a pretty short document.")
    return doc

nlp = spacy.load('en')
nlp.add_pipe(my_component, name='print_info', first=True)
print(nlp.pipe_names)  # ['print_info', 'tagger', 'parser', 'ner']
doc = nlp(u"This is a sentence.")
```