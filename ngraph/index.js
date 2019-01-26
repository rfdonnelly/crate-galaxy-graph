#!/usr/bin/env node

path = process.argv[2];

var fs = require('fs');
var dot_content = fs.readFileSync(path, 'utf8');

var dot = require('ngraph.fromdot');
var graph = dot(dot_content);

var createLayout = require('ngraph.offline.layout');
var layout = createLayout(graph);
layout.run();

var save = require('ngraph.tobinary');
save(graph, {
      outDir: './data'
});
