# MonoBlog

Yet another static site generator for who cares hyphenation in Western words and space between CJK and Western parts.

MonoBlog insert U+2009 between Chinese and Western parts across inline tags, insert U+00AD in the appropriate place inside Western words according to Liang's Hyphenation algorithm and LaTeX's corpus.

# Usage

Process file in arg, or data from stdin

```
$ mblog ulysses.md
```

or

```
$ cat ulysses.md | mblog
```

# Format

Front matter and body are just plain markdown. Posts are joined with newline,
which is the default format exported from [Ulysses](https://ulyssesapp.com).

## LICENSE

The MIT License

Copyright (c) 2018 Daniel

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
