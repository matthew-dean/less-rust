const css = require('@less/tree-sitter-css/grammar')

module.exports = grammar(css, {
  name: 'less',
  rules: {
    source_file: $ => 'hello'
  }
})