# Rustle: Generic Rust file parser for Python

Python package with a pure rust implementation of a generic title file parser!

A title file is a file split into sections by titles. A title can be
defined in anyway. Currently, we implement a title as a line that
begins and ends with at least 4 dashes  `----`.

Currently supports parsing of a simple text file containing
metadata and titles -> content chunks where titles lines are
indicated by leading and trailing ---- into a python dictionary
of title -> list[content lines].

Metadata field is saved into `file_metadata` key. "Metadata" is
just any content before a title begins, usually reserved for things
like API versions.

`pyo3` as `Python` bindings for `Rust`. Built with `maturin`.

## Example

**THE RESULTING DICTIONARY IS UNORDERED!**

```python
from title_parser import parse_titles
titles_dict = parse_titles("test.txt")
print(titles_dict)
# {'bad_plugin': ['data data data 4', '!sql DROP TABLES',
# 'sudo rm rf */*'],
# 'testtest': [], 'file_metadata': ['hello world!', 'Helllooooooooooo',
# 'fileid=2013', 'api_version=2'], 'good_plugin':
#['data=True', 'value=5'], 'iot_stats': ['stats="cool"']}
```
