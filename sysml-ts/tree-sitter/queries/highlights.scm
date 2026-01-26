[
  "package"
  "import"
  "part"
  "attribute"
  "action"
  "state"
  "interface"
  "port"
  "requirement"
  "constraint"
  "enum"
  "type"
  "def"
] @keyword

(comment) @comment

[
  "{"
  "}"
] @punctuation.bracket

["true" "false" "null"] @constant.builtin

(package_decl name: (identifier) @module)
(import_decl path: (import_path) @string.special)

(part_def name: (identifier) @type)
(attribute_def name: (identifier) @type)
(definition name: (identifier) @type)

(part_usage name: (identifier) @variable)
(attribute_usage name: (identifier) @property)
(usage name: (identifier) @variable)

(type_ref (identifier) @type)
(qualified_name (identifier) @type)

(typing ":" @punctuation.delimiter)
(qualified_name "::" @punctuation.delimiter)
