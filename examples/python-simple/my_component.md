Custom components can be added to the pipeline using the `add_pipe`  method. Optionally, you can either specify a component to add it before or after, tell spaCy to add it first or last in the pipeline, or define a custom name. If no name is set and no name attribute is present on your component, the function name is used.

```python
{{< id="my_component" src="examples/python-simple/my_component.py" >}}
```