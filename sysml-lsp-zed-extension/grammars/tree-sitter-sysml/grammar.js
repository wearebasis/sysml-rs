module.exports = grammar({
  name: 'sysml',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($._statement),

    _statement: $ => choice(
      $.package_decl,
      $.part_def,
      $.attribute_decl,
      $.import_decl,
      $.definition
    ),

    block: $ => seq(
      '{',
      repeat($._statement),
      '}'
    ),

    package_decl: $ => seq(
      'package',
      field('name', $.identifier),
      optional($.block)
    ),

    part_def: $ => seq(
      'part',
      optional('def'),
      field('name', $.identifier),
      optional($.typing),
      optional($.block),
      optional(';')
    ),

    attribute_decl: $ => seq(
      'attribute',
      field('name', $.identifier),
      optional($.typing),
      optional(';')
    ),

    import_decl: $ => seq(
      'import',
      /[^;]+/,
      ';'
    ),

    definition: $ => seq(
      choice('action', 'state', 'interface', 'port', 'requirement', 'constraint', 'enum', 'type'),
      optional('def'),
      field('name', $.identifier),
      optional($.block),
      optional(';')
    ),

    typing: $ => seq(':', field('type', $.type_ref)),

    type_ref: $ => choice(
      $.qualified_name,
      $.identifier
    ),

    qualified_name: $ => seq(
      $.identifier,
      repeat1(seq('::', $.identifier))
    ),

    identifier: $ => /[A-Za-z_][A-Za-z0-9_]*/,

    string: $ => /"([^"\\]|\\.)*"/,

    number: $ => /\d+(\.\d+)?/,

    comment: $ => token(choice(
      seq('//', /.*/),
      /\/\*[^*]*\*+([^/*][^*]*\*+)*\//
    )),
  }
});
