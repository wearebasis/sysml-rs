const keywords = require("../generated/keywords");
const operators = require("../generated/operators");
const enums = require("../generated/enums");

const operatorSymbols = Array.from(
  new Set(operators.flatMap((op) => op.symbols))
).sort((a, b) => b.length - a.length);

const enumValues = Array.from(
  new Set(Object.values(enums).flat())
).sort();

module.exports = grammar({
  name: "sysml",

  extras: ($) => [/\s/, $.comment],

  word: ($) => $.identifier,

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) =>
      choice(
        $.package_decl,
        $.import_decl,
        $.part_def,
        $.part_usage,
        $.attribute_def,
        $.attribute_usage,
        $.definition,
        $.usage
      ),

    block: ($) => seq("{", repeat($._statement), "}"),

    package_decl: ($) =>
      seq("package", field("name", $.identifier), optional($.block)),

    import_decl: ($) =>
      seq("import", field("path", $.import_path), ";"),

    import_path: ($) => /[^;]+/,

    part_def: ($) =>
      prec(
        2,
        seq(
          "part",
          "def",
          field("name", $.identifier),
          optional($.typing),
          optional($.block),
          optional(";")
        )
      ),

    part_usage: ($) =>
      prec(
        1,
        seq(
          "part",
          field("name", $.identifier),
          optional($.typing),
          optional($.block),
          optional(";")
        )
      ),

    attribute_def: ($) =>
      prec(
        2,
        seq(
          "attribute",
          "def",
          field("name", $.identifier),
          optional($.typing),
          optional(";")
        )
      ),

    attribute_usage: ($) =>
      prec(
        1,
        seq(
          "attribute",
          field("name", $.identifier),
          optional($.typing),
          optional(";")
        )
      ),

    definition: ($) =>
      prec(
        2,
        seq(
          choice(
            "action",
            "state",
            "interface",
            "port",
            "requirement",
            "constraint",
            "enum",
            "type"
          ),
          "def",
          field("name", $.identifier),
          optional($.typing),
          optional($.block),
          optional(";")
        )
      ),

    usage: ($) =>
      prec(
        1,
        seq(
          choice(
            "action",
            "state",
            "interface",
            "port",
            "requirement",
            "constraint",
            "enum",
            "type"
          ),
          field("name", $.identifier),
          optional($.typing),
          optional($.block),
          optional(";")
        )
      ),

    typing: ($) => seq(":", field("type", $.type_ref)),

    type_ref: ($) => choice($.qualified_name, $.identifier),

    qualified_name: ($) =>
      seq($.identifier, repeat1(seq("::", $.identifier))),

    identifier: ($) => token(/[A-Za-z_][A-Za-z0-9_]*/),

    literal: ($) => choice($.string, $.number, $.boolean, $.null),

    string: ($) => /"([^"\\]|\\.)*"/,

    number: ($) => /\d+(\.\d+)?/,

    boolean: ($) => choice("true", "false"),

    null: ($) => "null",

    keyword: ($) => choice(...keywords),

    operator: ($) => choice(...operatorSymbols),

    enum_value: ($) => choice(...enumValues),

    comment: ($) =>
      token(choice(seq("//", /.*/), /\/\*[^*]*\*+([^/*][^*]*\*+)*\//)),
  },
});
