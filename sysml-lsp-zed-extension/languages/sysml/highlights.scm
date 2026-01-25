[
  "package"
  "part"
  "def"
  "attribute"
  "import"
  "action"
  "state"
  "interface"
  "port"
  "requirement"
  "constraint"
  "enum"
  "type"
  "item"
  "unit"
  "calc"
  "analysis"
  "assert"
  "if"
  "then"
  "else"
  "for"
  "in"
  "do"
  "return"
] @keyword

(comment) @comment
(string) @string
(number) @number

(type_ref (identifier) @type)
(type_ref (qualified_name) @type)
(identifier) @variable
