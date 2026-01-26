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
(string) @string
(number) @number

(type_ref (identifier) @type)
(type_ref (qualified_name) @type)
(identifier) @variable
