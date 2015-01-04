Very basic scripts for drawing a graph of the crates.io
packages. [Blog post](http://huonw.github.io/blog/2015/01/crates.io-dep-graph/).

`make` should create something semisensible, code may have to be
edited (the `MAX_REV_DEP_COUNT` field at the top of `src/main.rs`) to
reproduce the results shown in the blog post (well, to get closer: the
package system is living and changes).

It requires graphviz (`fdp` and `gvpr`) and `git` at least.
