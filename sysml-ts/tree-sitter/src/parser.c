#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 65
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 222
#define ALIAS_COUNT 0
#define TOKEN_COUNT 206
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 5

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  anon_sym_package = 4,
  anon_sym_import = 5,
  anon_sym_SEMI = 6,
  sym_import_path = 7,
  anon_sym_part = 8,
  anon_sym_def = 9,
  anon_sym_attribute = 10,
  anon_sym_action = 11,
  anon_sym_state = 12,
  anon_sym_interface = 13,
  anon_sym_port = 14,
  anon_sym_requirement = 15,
  anon_sym_constraint = 16,
  anon_sym_enum = 17,
  anon_sym_type = 18,
  anon_sym_COLON = 19,
  anon_sym_COLON_COLON = 20,
  sym_string = 21,
  sym_number = 22,
  anon_sym_true = 23,
  anon_sym_false = 24,
  anon_sym_null = 25,
  anon_sym_about = 26,
  anon_sym_abstract = 27,
  anon_sym_accept = 28,
  anon_sym_actor = 29,
  anon_sym_after = 30,
  anon_sym_alias = 31,
  anon_sym_all = 32,
  anon_sym_allocate = 33,
  anon_sym_allocation = 34,
  anon_sym_analysis = 35,
  anon_sym_and = 36,
  anon_sym_as = 37,
  anon_sym_assert = 38,
  anon_sym_assign = 39,
  anon_sym_assoc = 40,
  anon_sym_assume = 41,
  anon_sym_at = 42,
  anon_sym_behavior = 43,
  anon_sym_bind = 44,
  anon_sym_binding = 45,
  anon_sym_bool = 46,
  anon_sym_by = 47,
  anon_sym_calc = 48,
  anon_sym_case = 49,
  anon_sym_chains = 50,
  anon_sym_class = 51,
  anon_sym_classifier = 52,
  anon_sym_comment = 53,
  anon_sym_composite = 54,
  anon_sym_concern = 55,
  anon_sym_conjugate = 56,
  anon_sym_conjugates = 57,
  anon_sym_conjugation = 58,
  anon_sym_connect = 59,
  anon_sym_connection = 60,
  anon_sym_connector = 61,
  anon_sym_const = 62,
  anon_sym_constant = 63,
  anon_sym_crosses = 64,
  anon_sym_datatype = 65,
  anon_sym_decide = 66,
  anon_sym_default = 67,
  anon_sym_defined = 68,
  anon_sym_dependency = 69,
  anon_sym_derived = 70,
  anon_sym_differences = 71,
  anon_sym_disjoining = 72,
  anon_sym_disjoint = 73,
  anon_sym_do = 74,
  anon_sym_doc = 75,
  anon_sym_else = 76,
  anon_sym_end = 77,
  anon_sym_entry = 78,
  anon_sym_event = 79,
  anon_sym_exhibit = 80,
  anon_sym_exit = 81,
  anon_sym_expose = 82,
  anon_sym_expr = 83,
  anon_sym_feature = 84,
  anon_sym_featured = 85,
  anon_sym_featuring = 86,
  anon_sym_filter = 87,
  anon_sym_first = 88,
  anon_sym_flow = 89,
  anon_sym_for = 90,
  anon_sym_fork = 91,
  anon_sym_frame = 92,
  anon_sym_from = 93,
  anon_sym_function = 94,
  anon_sym_hastype = 95,
  anon_sym_if = 96,
  anon_sym_implies = 97,
  anon_sym_in = 98,
  anon_sym_include = 99,
  anon_sym_individual = 100,
  anon_sym_inout = 101,
  anon_sym_interaction = 102,
  anon_sym_intersects = 103,
  anon_sym_inv = 104,
  anon_sym_inverse = 105,
  anon_sym_inverting = 106,
  anon_sym_istype = 107,
  anon_sym_item = 108,
  anon_sym_join = 109,
  anon_sym_language = 110,
  anon_sym_library = 111,
  anon_sym_locale = 112,
  anon_sym_loop = 113,
  anon_sym_member = 114,
  anon_sym_merge = 115,
  anon_sym_message = 116,
  anon_sym_meta = 117,
  anon_sym_metaclass = 118,
  anon_sym_metadata = 119,
  anon_sym_multiplicity = 120,
  anon_sym_namespace = 121,
  anon_sym_new = 122,
  anon_sym_nonunique = 123,
  anon_sym_not = 124,
  anon_sym_objective = 125,
  anon_sym_occurrence = 126,
  anon_sym_of = 127,
  anon_sym_or = 128,
  anon_sym_ordered = 129,
  anon_sym_out = 130,
  anon_sym_parallel = 131,
  anon_sym_perform = 132,
  anon_sym_portion = 133,
  anon_sym_predicate = 134,
  anon_sym_private = 135,
  anon_sym_protected = 136,
  anon_sym_public = 137,
  anon_sym_readonly = 138,
  anon_sym_redefines = 139,
  anon_sym_redefinition = 140,
  anon_sym_ref = 141,
  anon_sym_references = 142,
  anon_sym_render = 143,
  anon_sym_rendering = 144,
  anon_sym_rep = 145,
  anon_sym_require = 146,
  anon_sym_return = 147,
  anon_sym_satisfy = 148,
  anon_sym_send = 149,
  anon_sym_snapshot = 150,
  anon_sym_specialization = 151,
  anon_sym_specializes = 152,
  anon_sym_stakeholder = 153,
  anon_sym_standard = 154,
  anon_sym_step = 155,
  anon_sym_struct = 156,
  anon_sym_subclassifier = 157,
  anon_sym_subject = 158,
  anon_sym_subset = 159,
  anon_sym_subsets = 160,
  anon_sym_subtype = 161,
  anon_sym_succession = 162,
  anon_sym_terminate = 163,
  anon_sym_then = 164,
  anon_sym_timeslice = 165,
  anon_sym_to = 166,
  anon_sym_transition = 167,
  anon_sym_typed = 168,
  anon_sym_typing = 169,
  anon_sym_unions = 170,
  anon_sym_until = 171,
  anon_sym_use = 172,
  anon_sym_var = 173,
  anon_sym_variant = 174,
  anon_sym_variation = 175,
  anon_sym_verification = 176,
  anon_sym_verify = 177,
  anon_sym_via = 178,
  anon_sym_view = 179,
  anon_sym_viewpoint = 180,
  anon_sym_when = 181,
  anon_sym_while = 182,
  anon_sym_xor = 183,
  anon_sym_EQ_EQ_EQ = 184,
  anon_sym_BANG_EQ_EQ = 185,
  anon_sym_QMARK_QMARK = 186,
  anon_sym_EQ_EQ = 187,
  anon_sym_BANG_EQ = 188,
  anon_sym_AT_AT = 189,
  anon_sym_LT_EQ = 190,
  anon_sym_GT_EQ = 191,
  anon_sym_STAR_STAR = 192,
  anon_sym_PIPE = 193,
  anon_sym_AMP = 194,
  anon_sym_AT = 195,
  anon_sym_LT = 196,
  anon_sym_GT = 197,
  anon_sym_PLUS = 198,
  anon_sym_DASH = 199,
  anon_sym_STAR = 200,
  anon_sym_SLASH = 201,
  anon_sym_PERCENT = 202,
  anon_sym_CARET = 203,
  anon_sym_TILDE = 204,
  sym_comment = 205,
  sym_source_file = 206,
  sym__statement = 207,
  sym_block = 208,
  sym_package_decl = 209,
  sym_import_decl = 210,
  sym_part_def = 211,
  sym_part_usage = 212,
  sym_attribute_def = 213,
  sym_attribute_usage = 214,
  sym_definition = 215,
  sym_usage = 216,
  sym_typing = 217,
  sym_type_ref = 218,
  sym_qualified_name = 219,
  aux_sym_source_file_repeat1 = 220,
  aux_sym_qualified_name_repeat1 = 221,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_package] = "package",
  [anon_sym_import] = "import",
  [anon_sym_SEMI] = ";",
  [sym_import_path] = "import_path",
  [anon_sym_part] = "part",
  [anon_sym_def] = "def",
  [anon_sym_attribute] = "attribute",
  [anon_sym_action] = "action",
  [anon_sym_state] = "state",
  [anon_sym_interface] = "interface",
  [anon_sym_port] = "port",
  [anon_sym_requirement] = "requirement",
  [anon_sym_constraint] = "constraint",
  [anon_sym_enum] = "enum",
  [anon_sym_type] = "type",
  [anon_sym_COLON] = ":",
  [anon_sym_COLON_COLON] = "::",
  [sym_string] = "string",
  [sym_number] = "number",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [anon_sym_null] = "null",
  [anon_sym_about] = "about",
  [anon_sym_abstract] = "abstract",
  [anon_sym_accept] = "accept",
  [anon_sym_actor] = "actor",
  [anon_sym_after] = "after",
  [anon_sym_alias] = "alias",
  [anon_sym_all] = "all",
  [anon_sym_allocate] = "allocate",
  [anon_sym_allocation] = "allocation",
  [anon_sym_analysis] = "analysis",
  [anon_sym_and] = "and",
  [anon_sym_as] = "as",
  [anon_sym_assert] = "assert",
  [anon_sym_assign] = "assign",
  [anon_sym_assoc] = "assoc",
  [anon_sym_assume] = "assume",
  [anon_sym_at] = "at",
  [anon_sym_behavior] = "behavior",
  [anon_sym_bind] = "bind",
  [anon_sym_binding] = "binding",
  [anon_sym_bool] = "bool",
  [anon_sym_by] = "by",
  [anon_sym_calc] = "calc",
  [anon_sym_case] = "case",
  [anon_sym_chains] = "chains",
  [anon_sym_class] = "class",
  [anon_sym_classifier] = "classifier",
  [anon_sym_comment] = "comment",
  [anon_sym_composite] = "composite",
  [anon_sym_concern] = "concern",
  [anon_sym_conjugate] = "conjugate",
  [anon_sym_conjugates] = "conjugates",
  [anon_sym_conjugation] = "conjugation",
  [anon_sym_connect] = "connect",
  [anon_sym_connection] = "connection",
  [anon_sym_connector] = "connector",
  [anon_sym_const] = "const",
  [anon_sym_constant] = "constant",
  [anon_sym_crosses] = "crosses",
  [anon_sym_datatype] = "datatype",
  [anon_sym_decide] = "decide",
  [anon_sym_default] = "default",
  [anon_sym_defined] = "defined",
  [anon_sym_dependency] = "dependency",
  [anon_sym_derived] = "derived",
  [anon_sym_differences] = "differences",
  [anon_sym_disjoining] = "disjoining",
  [anon_sym_disjoint] = "disjoint",
  [anon_sym_do] = "do",
  [anon_sym_doc] = "doc",
  [anon_sym_else] = "else",
  [anon_sym_end] = "end",
  [anon_sym_entry] = "entry",
  [anon_sym_event] = "event",
  [anon_sym_exhibit] = "exhibit",
  [anon_sym_exit] = "exit",
  [anon_sym_expose] = "expose",
  [anon_sym_expr] = "expr",
  [anon_sym_feature] = "feature",
  [anon_sym_featured] = "featured",
  [anon_sym_featuring] = "featuring",
  [anon_sym_filter] = "filter",
  [anon_sym_first] = "first",
  [anon_sym_flow] = "flow",
  [anon_sym_for] = "for",
  [anon_sym_fork] = "fork",
  [anon_sym_frame] = "frame",
  [anon_sym_from] = "from",
  [anon_sym_function] = "function",
  [anon_sym_hastype] = "hastype",
  [anon_sym_if] = "if",
  [anon_sym_implies] = "implies",
  [anon_sym_in] = "in",
  [anon_sym_include] = "include",
  [anon_sym_individual] = "individual",
  [anon_sym_inout] = "inout",
  [anon_sym_interaction] = "interaction",
  [anon_sym_intersects] = "intersects",
  [anon_sym_inv] = "inv",
  [anon_sym_inverse] = "inverse",
  [anon_sym_inverting] = "inverting",
  [anon_sym_istype] = "istype",
  [anon_sym_item] = "item",
  [anon_sym_join] = "join",
  [anon_sym_language] = "language",
  [anon_sym_library] = "library",
  [anon_sym_locale] = "locale",
  [anon_sym_loop] = "loop",
  [anon_sym_member] = "member",
  [anon_sym_merge] = "merge",
  [anon_sym_message] = "message",
  [anon_sym_meta] = "meta",
  [anon_sym_metaclass] = "metaclass",
  [anon_sym_metadata] = "metadata",
  [anon_sym_multiplicity] = "multiplicity",
  [anon_sym_namespace] = "namespace",
  [anon_sym_new] = "new",
  [anon_sym_nonunique] = "nonunique",
  [anon_sym_not] = "not",
  [anon_sym_objective] = "objective",
  [anon_sym_occurrence] = "occurrence",
  [anon_sym_of] = "of",
  [anon_sym_or] = "or",
  [anon_sym_ordered] = "ordered",
  [anon_sym_out] = "out",
  [anon_sym_parallel] = "parallel",
  [anon_sym_perform] = "perform",
  [anon_sym_portion] = "portion",
  [anon_sym_predicate] = "predicate",
  [anon_sym_private] = "private",
  [anon_sym_protected] = "protected",
  [anon_sym_public] = "public",
  [anon_sym_readonly] = "readonly",
  [anon_sym_redefines] = "redefines",
  [anon_sym_redefinition] = "redefinition",
  [anon_sym_ref] = "ref",
  [anon_sym_references] = "references",
  [anon_sym_render] = "render",
  [anon_sym_rendering] = "rendering",
  [anon_sym_rep] = "rep",
  [anon_sym_require] = "require",
  [anon_sym_return] = "return",
  [anon_sym_satisfy] = "satisfy",
  [anon_sym_send] = "send",
  [anon_sym_snapshot] = "snapshot",
  [anon_sym_specialization] = "specialization",
  [anon_sym_specializes] = "specializes",
  [anon_sym_stakeholder] = "stakeholder",
  [anon_sym_standard] = "standard",
  [anon_sym_step] = "step",
  [anon_sym_struct] = "struct",
  [anon_sym_subclassifier] = "subclassifier",
  [anon_sym_subject] = "subject",
  [anon_sym_subset] = "subset",
  [anon_sym_subsets] = "subsets",
  [anon_sym_subtype] = "subtype",
  [anon_sym_succession] = "succession",
  [anon_sym_terminate] = "terminate",
  [anon_sym_then] = "then",
  [anon_sym_timeslice] = "timeslice",
  [anon_sym_to] = "to",
  [anon_sym_transition] = "transition",
  [anon_sym_typed] = "typed",
  [anon_sym_typing] = "typing",
  [anon_sym_unions] = "unions",
  [anon_sym_until] = "until",
  [anon_sym_use] = "use",
  [anon_sym_var] = "var",
  [anon_sym_variant] = "variant",
  [anon_sym_variation] = "variation",
  [anon_sym_verification] = "verification",
  [anon_sym_verify] = "verify",
  [anon_sym_via] = "via",
  [anon_sym_view] = "view",
  [anon_sym_viewpoint] = "viewpoint",
  [anon_sym_when] = "when",
  [anon_sym_while] = "while",
  [anon_sym_xor] = "xor",
  [anon_sym_EQ_EQ_EQ] = "===",
  [anon_sym_BANG_EQ_EQ] = "!==",
  [anon_sym_QMARK_QMARK] = "\?\?",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_AT_AT] = "@@",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_STAR_STAR] = "**",
  [anon_sym_PIPE] = "|",
  [anon_sym_AMP] = "&",
  [anon_sym_AT] = "@",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_CARET] = "^",
  [anon_sym_TILDE] = "~",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym__statement] = "_statement",
  [sym_block] = "block",
  [sym_package_decl] = "package_decl",
  [sym_import_decl] = "import_decl",
  [sym_part_def] = "part_def",
  [sym_part_usage] = "part_usage",
  [sym_attribute_def] = "attribute_def",
  [sym_attribute_usage] = "attribute_usage",
  [sym_definition] = "definition",
  [sym_usage] = "usage",
  [sym_typing] = "typing",
  [sym_type_ref] = "type_ref",
  [sym_qualified_name] = "qualified_name",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_qualified_name_repeat1] = "qualified_name_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_package] = anon_sym_package,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_import_path] = sym_import_path,
  [anon_sym_part] = anon_sym_part,
  [anon_sym_def] = anon_sym_def,
  [anon_sym_attribute] = anon_sym_attribute,
  [anon_sym_action] = anon_sym_action,
  [anon_sym_state] = anon_sym_state,
  [anon_sym_interface] = anon_sym_interface,
  [anon_sym_port] = anon_sym_port,
  [anon_sym_requirement] = anon_sym_requirement,
  [anon_sym_constraint] = anon_sym_constraint,
  [anon_sym_enum] = anon_sym_enum,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [sym_string] = sym_string,
  [sym_number] = sym_number,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [anon_sym_null] = anon_sym_null,
  [anon_sym_about] = anon_sym_about,
  [anon_sym_abstract] = anon_sym_abstract,
  [anon_sym_accept] = anon_sym_accept,
  [anon_sym_actor] = anon_sym_actor,
  [anon_sym_after] = anon_sym_after,
  [anon_sym_alias] = anon_sym_alias,
  [anon_sym_all] = anon_sym_all,
  [anon_sym_allocate] = anon_sym_allocate,
  [anon_sym_allocation] = anon_sym_allocation,
  [anon_sym_analysis] = anon_sym_analysis,
  [anon_sym_and] = anon_sym_and,
  [anon_sym_as] = anon_sym_as,
  [anon_sym_assert] = anon_sym_assert,
  [anon_sym_assign] = anon_sym_assign,
  [anon_sym_assoc] = anon_sym_assoc,
  [anon_sym_assume] = anon_sym_assume,
  [anon_sym_at] = anon_sym_at,
  [anon_sym_behavior] = anon_sym_behavior,
  [anon_sym_bind] = anon_sym_bind,
  [anon_sym_binding] = anon_sym_binding,
  [anon_sym_bool] = anon_sym_bool,
  [anon_sym_by] = anon_sym_by,
  [anon_sym_calc] = anon_sym_calc,
  [anon_sym_case] = anon_sym_case,
  [anon_sym_chains] = anon_sym_chains,
  [anon_sym_class] = anon_sym_class,
  [anon_sym_classifier] = anon_sym_classifier,
  [anon_sym_comment] = anon_sym_comment,
  [anon_sym_composite] = anon_sym_composite,
  [anon_sym_concern] = anon_sym_concern,
  [anon_sym_conjugate] = anon_sym_conjugate,
  [anon_sym_conjugates] = anon_sym_conjugates,
  [anon_sym_conjugation] = anon_sym_conjugation,
  [anon_sym_connect] = anon_sym_connect,
  [anon_sym_connection] = anon_sym_connection,
  [anon_sym_connector] = anon_sym_connector,
  [anon_sym_const] = anon_sym_const,
  [anon_sym_constant] = anon_sym_constant,
  [anon_sym_crosses] = anon_sym_crosses,
  [anon_sym_datatype] = anon_sym_datatype,
  [anon_sym_decide] = anon_sym_decide,
  [anon_sym_default] = anon_sym_default,
  [anon_sym_defined] = anon_sym_defined,
  [anon_sym_dependency] = anon_sym_dependency,
  [anon_sym_derived] = anon_sym_derived,
  [anon_sym_differences] = anon_sym_differences,
  [anon_sym_disjoining] = anon_sym_disjoining,
  [anon_sym_disjoint] = anon_sym_disjoint,
  [anon_sym_do] = anon_sym_do,
  [anon_sym_doc] = anon_sym_doc,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_end] = anon_sym_end,
  [anon_sym_entry] = anon_sym_entry,
  [anon_sym_event] = anon_sym_event,
  [anon_sym_exhibit] = anon_sym_exhibit,
  [anon_sym_exit] = anon_sym_exit,
  [anon_sym_expose] = anon_sym_expose,
  [anon_sym_expr] = anon_sym_expr,
  [anon_sym_feature] = anon_sym_feature,
  [anon_sym_featured] = anon_sym_featured,
  [anon_sym_featuring] = anon_sym_featuring,
  [anon_sym_filter] = anon_sym_filter,
  [anon_sym_first] = anon_sym_first,
  [anon_sym_flow] = anon_sym_flow,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_fork] = anon_sym_fork,
  [anon_sym_frame] = anon_sym_frame,
  [anon_sym_from] = anon_sym_from,
  [anon_sym_function] = anon_sym_function,
  [anon_sym_hastype] = anon_sym_hastype,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_implies] = anon_sym_implies,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_include] = anon_sym_include,
  [anon_sym_individual] = anon_sym_individual,
  [anon_sym_inout] = anon_sym_inout,
  [anon_sym_interaction] = anon_sym_interaction,
  [anon_sym_intersects] = anon_sym_intersects,
  [anon_sym_inv] = anon_sym_inv,
  [anon_sym_inverse] = anon_sym_inverse,
  [anon_sym_inverting] = anon_sym_inverting,
  [anon_sym_istype] = anon_sym_istype,
  [anon_sym_item] = anon_sym_item,
  [anon_sym_join] = anon_sym_join,
  [anon_sym_language] = anon_sym_language,
  [anon_sym_library] = anon_sym_library,
  [anon_sym_locale] = anon_sym_locale,
  [anon_sym_loop] = anon_sym_loop,
  [anon_sym_member] = anon_sym_member,
  [anon_sym_merge] = anon_sym_merge,
  [anon_sym_message] = anon_sym_message,
  [anon_sym_meta] = anon_sym_meta,
  [anon_sym_metaclass] = anon_sym_metaclass,
  [anon_sym_metadata] = anon_sym_metadata,
  [anon_sym_multiplicity] = anon_sym_multiplicity,
  [anon_sym_namespace] = anon_sym_namespace,
  [anon_sym_new] = anon_sym_new,
  [anon_sym_nonunique] = anon_sym_nonunique,
  [anon_sym_not] = anon_sym_not,
  [anon_sym_objective] = anon_sym_objective,
  [anon_sym_occurrence] = anon_sym_occurrence,
  [anon_sym_of] = anon_sym_of,
  [anon_sym_or] = anon_sym_or,
  [anon_sym_ordered] = anon_sym_ordered,
  [anon_sym_out] = anon_sym_out,
  [anon_sym_parallel] = anon_sym_parallel,
  [anon_sym_perform] = anon_sym_perform,
  [anon_sym_portion] = anon_sym_portion,
  [anon_sym_predicate] = anon_sym_predicate,
  [anon_sym_private] = anon_sym_private,
  [anon_sym_protected] = anon_sym_protected,
  [anon_sym_public] = anon_sym_public,
  [anon_sym_readonly] = anon_sym_readonly,
  [anon_sym_redefines] = anon_sym_redefines,
  [anon_sym_redefinition] = anon_sym_redefinition,
  [anon_sym_ref] = anon_sym_ref,
  [anon_sym_references] = anon_sym_references,
  [anon_sym_render] = anon_sym_render,
  [anon_sym_rendering] = anon_sym_rendering,
  [anon_sym_rep] = anon_sym_rep,
  [anon_sym_require] = anon_sym_require,
  [anon_sym_return] = anon_sym_return,
  [anon_sym_satisfy] = anon_sym_satisfy,
  [anon_sym_send] = anon_sym_send,
  [anon_sym_snapshot] = anon_sym_snapshot,
  [anon_sym_specialization] = anon_sym_specialization,
  [anon_sym_specializes] = anon_sym_specializes,
  [anon_sym_stakeholder] = anon_sym_stakeholder,
  [anon_sym_standard] = anon_sym_standard,
  [anon_sym_step] = anon_sym_step,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_subclassifier] = anon_sym_subclassifier,
  [anon_sym_subject] = anon_sym_subject,
  [anon_sym_subset] = anon_sym_subset,
  [anon_sym_subsets] = anon_sym_subsets,
  [anon_sym_subtype] = anon_sym_subtype,
  [anon_sym_succession] = anon_sym_succession,
  [anon_sym_terminate] = anon_sym_terminate,
  [anon_sym_then] = anon_sym_then,
  [anon_sym_timeslice] = anon_sym_timeslice,
  [anon_sym_to] = anon_sym_to,
  [anon_sym_transition] = anon_sym_transition,
  [anon_sym_typed] = anon_sym_typed,
  [anon_sym_typing] = anon_sym_typing,
  [anon_sym_unions] = anon_sym_unions,
  [anon_sym_until] = anon_sym_until,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_var] = anon_sym_var,
  [anon_sym_variant] = anon_sym_variant,
  [anon_sym_variation] = anon_sym_variation,
  [anon_sym_verification] = anon_sym_verification,
  [anon_sym_verify] = anon_sym_verify,
  [anon_sym_via] = anon_sym_via,
  [anon_sym_view] = anon_sym_view,
  [anon_sym_viewpoint] = anon_sym_viewpoint,
  [anon_sym_when] = anon_sym_when,
  [anon_sym_while] = anon_sym_while,
  [anon_sym_xor] = anon_sym_xor,
  [anon_sym_EQ_EQ_EQ] = anon_sym_EQ_EQ_EQ,
  [anon_sym_BANG_EQ_EQ] = anon_sym_BANG_EQ_EQ,
  [anon_sym_QMARK_QMARK] = anon_sym_QMARK_QMARK,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_AT_AT] = anon_sym_AT_AT,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_STAR_STAR] = anon_sym_STAR_STAR,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym__statement] = sym__statement,
  [sym_block] = sym_block,
  [sym_package_decl] = sym_package_decl,
  [sym_import_decl] = sym_import_decl,
  [sym_part_def] = sym_part_def,
  [sym_part_usage] = sym_part_usage,
  [sym_attribute_def] = sym_attribute_def,
  [sym_attribute_usage] = sym_attribute_usage,
  [sym_definition] = sym_definition,
  [sym_usage] = sym_usage,
  [sym_typing] = sym_typing,
  [sym_type_ref] = sym_type_ref,
  [sym_qualified_name] = sym_qualified_name,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_qualified_name_repeat1] = aux_sym_qualified_name_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_package] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [sym_import_path] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_part] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_def] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_attribute] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_action] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_state] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interface] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_port] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_requirement] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constraint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enum] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_null] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_about] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_abstract] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_accept] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_actor] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_after] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_alias] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_all] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_allocate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_allocation] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_analysis] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_and] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_as] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_assert] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_assign] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_assoc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_assume] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_at] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_behavior] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bind] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_binding] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bool] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_by] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_calc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_case] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_chains] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_class] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_classifier] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_comment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_composite] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_concern] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_conjugate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_conjugates] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_conjugation] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_connect] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_connection] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_connector] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_const] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constant] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_crosses] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_datatype] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_decide] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_default] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_defined] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dependency] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_derived] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_differences] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_disjoining] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_disjoint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_do] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_doc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_end] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_entry] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_event] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exhibit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_expose] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_expr] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_feature] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_featured] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_featuring] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_filter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_first] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_flow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_for] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fork] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_frame] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_from] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_function] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_hastype] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_implies] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_include] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_individual] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_inout] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interaction] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_intersects] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_inv] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_inverse] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_inverting] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_istype] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_item] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_join] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_language] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_library] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_locale] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_loop] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_member] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_merge] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_message] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_meta] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_metaclass] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_metadata] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_multiplicity] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_namespace] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_new] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nonunique] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_not] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_objective] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_occurrence] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_of] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_or] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ordered] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_out] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_parallel] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_perform] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_portion] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_predicate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_private] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_protected] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_public] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_readonly] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_redefines] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_redefinition] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ref] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_references] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_render] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rendering] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rep] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_require] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_satisfy] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_send] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_snapshot] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_specialization] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_specializes] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_stakeholder] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_standard] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_step] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subclassifier] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subject] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subset] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subsets] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subtype] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_succession] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_terminate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_then] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_timeslice] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_to] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_transition] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typed] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typing] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unions] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_until] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_var] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_variant] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_variation] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_verification] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_verify] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_via] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_view] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_viewpoint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_when] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_while] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_xor] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_QMARK_QMARK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_package_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_import_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_part_def] = {
    .visible = true,
    .named = true,
  },
  [sym_part_usage] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_def] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_usage] = {
    .visible = true,
    .named = true,
  },
  [sym_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_usage] = {
    .visible = true,
    .named = true,
  },
  [sym_typing] = {
    .visible = true,
    .named = true,
  },
  [sym_type_ref] = {
    .visible = true,
    .named = true,
  },
  [sym_qualified_name] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_qualified_name_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_name = 1,
  field_path = 2,
  field_type = 3,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_name] = "name",
  [field_path] = "path",
  [field_type] = "type",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 1},
  [1] =
    {field_path, 1},
  [2] =
    {field_name, 2},
  [3] =
    {field_type, 1},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(14);
      ADVANCE_MAP(
        '!', 7,
        '"', 1,
        '%', 49,
        '&', 41,
        '*', 47,
        '+', 45,
        '-', 46,
        '/', 48,
        ':', 25,
        ';', 17,
        '<', 43,
        '=', 8,
        '>', 44,
        '?', 9,
        '@', 42,
        '^', 50,
        '{', 15,
        '|', 40,
        '}', 16,
        '~', 51,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(28);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(53);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(52);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '/') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(22);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(23);
      END_STATE();
    case 6:
      if (lookahead == ':') ADVANCE(26);
      END_STATE();
    case 7:
      if (lookahead == '=') ADVANCE(35);
      END_STATE();
    case 8:
      if (lookahead == '=') ADVANCE(34);
      END_STATE();
    case 9:
      if (lookahead == '?') ADVANCE(33);
      END_STATE();
    case 10:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      END_STATE();
    case 11:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(1);
      END_STATE();
    case 12:
      if (eof) ADVANCE(14);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(24);
      if (lookahead == ';') ADVANCE(17);
      if (lookahead == '{') ADVANCE(15);
      if (lookahead == '}') ADVANCE(16);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(12);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 13:
      if (eof) ADVANCE(14);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(6);
      if (lookahead == ';') ADVANCE(17);
      if (lookahead == '{') ADVANCE(15);
      if (lookahead == '}') ADVANCE(16);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(13);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_import_path);
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead == ';') ADVANCE(53);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_import_path);
      if (lookahead == '*') ADVANCE(21);
      if (lookahead == '/') ADVANCE(18);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(23);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_import_path);
      if (lookahead == '*') ADVANCE(20);
      if (lookahead == '/') ADVANCE(23);
      if (lookahead == ';') ADVANCE(4);
      if (lookahead != 0) ADVANCE(21);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_import_path);
      if (lookahead == '*') ADVANCE(20);
      if (lookahead == ';') ADVANCE(4);
      if (lookahead != 0) ADVANCE(21);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_import_path);
      if (lookahead == '/') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(22);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(23);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_import_path);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(26);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(10);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_EQ_EQ_EQ);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_BANG_EQ_EQ);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_QMARK_QMARK);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      if (lookahead == '=') ADVANCE(31);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      if (lookahead == '=') ADVANCE(32);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_AT_AT);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_STAR_STAR);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead == '@') ADVANCE(36);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(37);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(38);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == '*') ADVANCE(39);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(53);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(53);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ADVANCE_MAP(
        'a', 1,
        'b', 2,
        'c', 3,
        'd', 4,
        'e', 5,
        'f', 6,
        'h', 7,
        'i', 8,
        'j', 9,
        'l', 10,
        'm', 11,
        'n', 12,
        'o', 13,
        'p', 14,
        'r', 15,
        's', 16,
        't', 17,
        'u', 18,
        'v', 19,
        'w', 20,
        'x', 21,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'b') ADVANCE(22);
      if (lookahead == 'c') ADVANCE(23);
      if (lookahead == 'f') ADVANCE(24);
      if (lookahead == 'l') ADVANCE(25);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 's') ADVANCE(27);
      if (lookahead == 't') ADVANCE(28);
      END_STATE();
    case 2:
      if (lookahead == 'e') ADVANCE(29);
      if (lookahead == 'i') ADVANCE(30);
      if (lookahead == 'o') ADVANCE(31);
      if (lookahead == 'y') ADVANCE(32);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(33);
      if (lookahead == 'h') ADVANCE(34);
      if (lookahead == 'l') ADVANCE(35);
      if (lookahead == 'o') ADVANCE(36);
      if (lookahead == 'r') ADVANCE(37);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(38);
      if (lookahead == 'e') ADVANCE(39);
      if (lookahead == 'i') ADVANCE(40);
      if (lookahead == 'o') ADVANCE(41);
      END_STATE();
    case 5:
      if (lookahead == 'l') ADVANCE(42);
      if (lookahead == 'n') ADVANCE(43);
      if (lookahead == 'v') ADVANCE(44);
      if (lookahead == 'x') ADVANCE(45);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(46);
      if (lookahead == 'e') ADVANCE(47);
      if (lookahead == 'i') ADVANCE(48);
      if (lookahead == 'l') ADVANCE(49);
      if (lookahead == 'o') ADVANCE(50);
      if (lookahead == 'r') ADVANCE(51);
      if (lookahead == 'u') ADVANCE(52);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(53);
      END_STATE();
    case 8:
      if (lookahead == 'f') ADVANCE(54);
      if (lookahead == 'm') ADVANCE(55);
      if (lookahead == 'n') ADVANCE(56);
      if (lookahead == 's') ADVANCE(57);
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 9:
      if (lookahead == 'o') ADVANCE(59);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(60);
      if (lookahead == 'i') ADVANCE(61);
      if (lookahead == 'o') ADVANCE(62);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(63);
      if (lookahead == 'u') ADVANCE(64);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(65);
      if (lookahead == 'e') ADVANCE(66);
      if (lookahead == 'o') ADVANCE(67);
      if (lookahead == 'u') ADVANCE(68);
      END_STATE();
    case 13:
      if (lookahead == 'b') ADVANCE(69);
      if (lookahead == 'c') ADVANCE(70);
      if (lookahead == 'f') ADVANCE(71);
      if (lookahead == 'r') ADVANCE(72);
      if (lookahead == 'u') ADVANCE(73);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(74);
      if (lookahead == 'e') ADVANCE(75);
      if (lookahead == 'o') ADVANCE(76);
      if (lookahead == 'r') ADVANCE(77);
      if (lookahead == 'u') ADVANCE(78);
      END_STATE();
    case 15:
      if (lookahead == 'e') ADVANCE(79);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(80);
      if (lookahead == 'e') ADVANCE(81);
      if (lookahead == 'n') ADVANCE(82);
      if (lookahead == 'p') ADVANCE(83);
      if (lookahead == 't') ADVANCE(84);
      if (lookahead == 'u') ADVANCE(85);
      END_STATE();
    case 17:
      if (lookahead == 'e') ADVANCE(86);
      if (lookahead == 'h') ADVANCE(87);
      if (lookahead == 'i') ADVANCE(88);
      if (lookahead == 'o') ADVANCE(89);
      if (lookahead == 'r') ADVANCE(90);
      if (lookahead == 'y') ADVANCE(91);
      END_STATE();
    case 18:
      if (lookahead == 'n') ADVANCE(92);
      if (lookahead == 's') ADVANCE(93);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(94);
      if (lookahead == 'e') ADVANCE(95);
      if (lookahead == 'i') ADVANCE(96);
      END_STATE();
    case 20:
      if (lookahead == 'h') ADVANCE(97);
      END_STATE();
    case 21:
      if (lookahead == 'o') ADVANCE(98);
      END_STATE();
    case 22:
      if (lookahead == 'o') ADVANCE(99);
      if (lookahead == 's') ADVANCE(100);
      END_STATE();
    case 23:
      if (lookahead == 'c') ADVANCE(101);
      if (lookahead == 't') ADVANCE(102);
      END_STATE();
    case 24:
      if (lookahead == 't') ADVANCE(103);
      END_STATE();
    case 25:
      if (lookahead == 'i') ADVANCE(104);
      if (lookahead == 'l') ADVANCE(105);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(106);
      if (lookahead == 'd') ADVANCE(107);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_as);
      if (lookahead == 's') ADVANCE(108);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_at);
      if (lookahead == 't') ADVANCE(109);
      END_STATE();
    case 29:
      if (lookahead == 'h') ADVANCE(110);
      END_STATE();
    case 30:
      if (lookahead == 'n') ADVANCE(111);
      END_STATE();
    case 31:
      if (lookahead == 'o') ADVANCE(112);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_by);
      END_STATE();
    case 33:
      if (lookahead == 'l') ADVANCE(113);
      if (lookahead == 's') ADVANCE(114);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(115);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(116);
      END_STATE();
    case 36:
      if (lookahead == 'm') ADVANCE(117);
      if (lookahead == 'n') ADVANCE(118);
      END_STATE();
    case 37:
      if (lookahead == 'o') ADVANCE(119);
      END_STATE();
    case 38:
      if (lookahead == 't') ADVANCE(120);
      END_STATE();
    case 39:
      if (lookahead == 'c') ADVANCE(121);
      if (lookahead == 'f') ADVANCE(122);
      if (lookahead == 'p') ADVANCE(123);
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 40:
      if (lookahead == 'f') ADVANCE(125);
      if (lookahead == 's') ADVANCE(126);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_do);
      if (lookahead == 'c') ADVANCE(127);
      END_STATE();
    case 42:
      if (lookahead == 's') ADVANCE(128);
      END_STATE();
    case 43:
      if (lookahead == 'd') ADVANCE(129);
      if (lookahead == 't') ADVANCE(130);
      if (lookahead == 'u') ADVANCE(131);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(132);
      END_STATE();
    case 45:
      if (lookahead == 'h') ADVANCE(133);
      if (lookahead == 'i') ADVANCE(134);
      if (lookahead == 'p') ADVANCE(135);
      END_STATE();
    case 46:
      if (lookahead == 'l') ADVANCE(136);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(137);
      END_STATE();
    case 48:
      if (lookahead == 'l') ADVANCE(138);
      if (lookahead == 'r') ADVANCE(139);
      END_STATE();
    case 49:
      if (lookahead == 'o') ADVANCE(140);
      END_STATE();
    case 50:
      if (lookahead == 'r') ADVANCE(141);
      END_STATE();
    case 51:
      if (lookahead == 'a') ADVANCE(142);
      if (lookahead == 'o') ADVANCE(143);
      END_STATE();
    case 52:
      if (lookahead == 'n') ADVANCE(144);
      END_STATE();
    case 53:
      if (lookahead == 's') ADVANCE(145);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 55:
      if (lookahead == 'p') ADVANCE(146);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'c') ADVANCE(147);
      if (lookahead == 'd') ADVANCE(148);
      if (lookahead == 'o') ADVANCE(149);
      if (lookahead == 't') ADVANCE(150);
      if (lookahead == 'v') ADVANCE(151);
      END_STATE();
    case 57:
      if (lookahead == 't') ADVANCE(152);
      END_STATE();
    case 58:
      if (lookahead == 'e') ADVANCE(153);
      END_STATE();
    case 59:
      if (lookahead == 'i') ADVANCE(154);
      END_STATE();
    case 60:
      if (lookahead == 'n') ADVANCE(155);
      END_STATE();
    case 61:
      if (lookahead == 'b') ADVANCE(156);
      END_STATE();
    case 62:
      if (lookahead == 'c') ADVANCE(157);
      if (lookahead == 'o') ADVANCE(158);
      END_STATE();
    case 63:
      if (lookahead == 'm') ADVANCE(159);
      if (lookahead == 'r') ADVANCE(160);
      if (lookahead == 's') ADVANCE(161);
      if (lookahead == 't') ADVANCE(162);
      END_STATE();
    case 64:
      if (lookahead == 'l') ADVANCE(163);
      END_STATE();
    case 65:
      if (lookahead == 'm') ADVANCE(164);
      END_STATE();
    case 66:
      if (lookahead == 'w') ADVANCE(165);
      END_STATE();
    case 67:
      if (lookahead == 'n') ADVANCE(166);
      if (lookahead == 't') ADVANCE(167);
      END_STATE();
    case 68:
      if (lookahead == 'l') ADVANCE(168);
      END_STATE();
    case 69:
      if (lookahead == 'j') ADVANCE(169);
      END_STATE();
    case 70:
      if (lookahead == 'c') ADVANCE(170);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_of);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_or);
      if (lookahead == 'd') ADVANCE(171);
      END_STATE();
    case 73:
      if (lookahead == 't') ADVANCE(172);
      END_STATE();
    case 74:
      if (lookahead == 'c') ADVANCE(173);
      if (lookahead == 'r') ADVANCE(174);
      END_STATE();
    case 75:
      if (lookahead == 'r') ADVANCE(175);
      END_STATE();
    case 76:
      if (lookahead == 'r') ADVANCE(176);
      END_STATE();
    case 77:
      if (lookahead == 'e') ADVANCE(177);
      if (lookahead == 'i') ADVANCE(178);
      if (lookahead == 'o') ADVANCE(179);
      END_STATE();
    case 78:
      if (lookahead == 'b') ADVANCE(180);
      END_STATE();
    case 79:
      if (lookahead == 'a') ADVANCE(181);
      if (lookahead == 'd') ADVANCE(182);
      if (lookahead == 'f') ADVANCE(183);
      if (lookahead == 'n') ADVANCE(184);
      if (lookahead == 'p') ADVANCE(185);
      if (lookahead == 'q') ADVANCE(186);
      if (lookahead == 't') ADVANCE(187);
      END_STATE();
    case 80:
      if (lookahead == 't') ADVANCE(188);
      END_STATE();
    case 81:
      if (lookahead == 'n') ADVANCE(189);
      END_STATE();
    case 82:
      if (lookahead == 'a') ADVANCE(190);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(191);
      END_STATE();
    case 84:
      if (lookahead == 'a') ADVANCE(192);
      if (lookahead == 'e') ADVANCE(193);
      if (lookahead == 'r') ADVANCE(194);
      END_STATE();
    case 85:
      if (lookahead == 'b') ADVANCE(195);
      if (lookahead == 'c') ADVANCE(196);
      END_STATE();
    case 86:
      if (lookahead == 'r') ADVANCE(197);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(198);
      END_STATE();
    case 88:
      if (lookahead == 'm') ADVANCE(199);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_to);
      END_STATE();
    case 90:
      if (lookahead == 'a') ADVANCE(200);
      if (lookahead == 'u') ADVANCE(201);
      END_STATE();
    case 91:
      if (lookahead == 'p') ADVANCE(202);
      END_STATE();
    case 92:
      if (lookahead == 'i') ADVANCE(203);
      if (lookahead == 't') ADVANCE(204);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(205);
      END_STATE();
    case 94:
      if (lookahead == 'r') ADVANCE(206);
      END_STATE();
    case 95:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 96:
      if (lookahead == 'a') ADVANCE(208);
      if (lookahead == 'e') ADVANCE(209);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(210);
      if (lookahead == 'i') ADVANCE(211);
      END_STATE();
    case 98:
      if (lookahead == 'r') ADVANCE(212);
      END_STATE();
    case 99:
      if (lookahead == 'u') ADVANCE(213);
      END_STATE();
    case 100:
      if (lookahead == 't') ADVANCE(214);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(215);
      END_STATE();
    case 102:
      if (lookahead == 'i') ADVANCE(216);
      if (lookahead == 'o') ADVANCE(217);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(218);
      END_STATE();
    case 104:
      if (lookahead == 'a') ADVANCE(219);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_all);
      if (lookahead == 'o') ADVANCE(220);
      END_STATE();
    case 106:
      if (lookahead == 'l') ADVANCE(221);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_and);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(222);
      if (lookahead == 'i') ADVANCE(223);
      if (lookahead == 'o') ADVANCE(224);
      if (lookahead == 'u') ADVANCE(225);
      END_STATE();
    case 109:
      if (lookahead == 'r') ADVANCE(226);
      END_STATE();
    case 110:
      if (lookahead == 'a') ADVANCE(227);
      END_STATE();
    case 111:
      if (lookahead == 'd') ADVANCE(228);
      END_STATE();
    case 112:
      if (lookahead == 'l') ADVANCE(229);
      END_STATE();
    case 113:
      if (lookahead == 'c') ADVANCE(230);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(231);
      END_STATE();
    case 115:
      if (lookahead == 'i') ADVANCE(232);
      END_STATE();
    case 116:
      if (lookahead == 's') ADVANCE(233);
      END_STATE();
    case 117:
      if (lookahead == 'm') ADVANCE(234);
      if (lookahead == 'p') ADVANCE(235);
      END_STATE();
    case 118:
      if (lookahead == 'c') ADVANCE(236);
      if (lookahead == 'j') ADVANCE(237);
      if (lookahead == 'n') ADVANCE(238);
      if (lookahead == 's') ADVANCE(239);
      END_STATE();
    case 119:
      if (lookahead == 's') ADVANCE(240);
      END_STATE();
    case 120:
      if (lookahead == 'a') ADVANCE(241);
      END_STATE();
    case 121:
      if (lookahead == 'i') ADVANCE(242);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_def);
      if (lookahead == 'a') ADVANCE(243);
      if (lookahead == 'i') ADVANCE(244);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 124:
      if (lookahead == 'i') ADVANCE(246);
      END_STATE();
    case 125:
      if (lookahead == 'f') ADVANCE(247);
      END_STATE();
    case 126:
      if (lookahead == 'j') ADVANCE(248);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_doc);
      END_STATE();
    case 128:
      if (lookahead == 'e') ADVANCE(249);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_end);
      END_STATE();
    case 130:
      if (lookahead == 'r') ADVANCE(250);
      END_STATE();
    case 131:
      if (lookahead == 'm') ADVANCE(251);
      END_STATE();
    case 132:
      if (lookahead == 'n') ADVANCE(252);
      END_STATE();
    case 133:
      if (lookahead == 'i') ADVANCE(253);
      END_STATE();
    case 134:
      if (lookahead == 't') ADVANCE(254);
      END_STATE();
    case 135:
      if (lookahead == 'o') ADVANCE(255);
      if (lookahead == 'r') ADVANCE(256);
      END_STATE();
    case 136:
      if (lookahead == 's') ADVANCE(257);
      END_STATE();
    case 137:
      if (lookahead == 't') ADVANCE(258);
      END_STATE();
    case 138:
      if (lookahead == 't') ADVANCE(259);
      END_STATE();
    case 139:
      if (lookahead == 's') ADVANCE(260);
      END_STATE();
    case 140:
      if (lookahead == 'w') ADVANCE(261);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(anon_sym_for);
      if (lookahead == 'k') ADVANCE(262);
      END_STATE();
    case 142:
      if (lookahead == 'm') ADVANCE(263);
      END_STATE();
    case 143:
      if (lookahead == 'm') ADVANCE(264);
      END_STATE();
    case 144:
      if (lookahead == 'c') ADVANCE(265);
      END_STATE();
    case 145:
      if (lookahead == 't') ADVANCE(266);
      END_STATE();
    case 146:
      if (lookahead == 'l') ADVANCE(267);
      if (lookahead == 'o') ADVANCE(268);
      END_STATE();
    case 147:
      if (lookahead == 'l') ADVANCE(269);
      END_STATE();
    case 148:
      if (lookahead == 'i') ADVANCE(270);
      END_STATE();
    case 149:
      if (lookahead == 'u') ADVANCE(271);
      END_STATE();
    case 150:
      if (lookahead == 'e') ADVANCE(272);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_inv);
      if (lookahead == 'e') ADVANCE(273);
      END_STATE();
    case 152:
      if (lookahead == 'y') ADVANCE(274);
      END_STATE();
    case 153:
      if (lookahead == 'm') ADVANCE(275);
      END_STATE();
    case 154:
      if (lookahead == 'n') ADVANCE(276);
      END_STATE();
    case 155:
      if (lookahead == 'g') ADVANCE(277);
      END_STATE();
    case 156:
      if (lookahead == 'r') ADVANCE(278);
      END_STATE();
    case 157:
      if (lookahead == 'a') ADVANCE(279);
      END_STATE();
    case 158:
      if (lookahead == 'p') ADVANCE(280);
      END_STATE();
    case 159:
      if (lookahead == 'b') ADVANCE(281);
      END_STATE();
    case 160:
      if (lookahead == 'g') ADVANCE(282);
      END_STATE();
    case 161:
      if (lookahead == 's') ADVANCE(283);
      END_STATE();
    case 162:
      if (lookahead == 'a') ADVANCE(284);
      END_STATE();
    case 163:
      if (lookahead == 't') ADVANCE(285);
      END_STATE();
    case 164:
      if (lookahead == 'e') ADVANCE(286);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(anon_sym_new);
      END_STATE();
    case 166:
      if (lookahead == 'u') ADVANCE(287);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(anon_sym_not);
      END_STATE();
    case 168:
      if (lookahead == 'l') ADVANCE(288);
      END_STATE();
    case 169:
      if (lookahead == 'e') ADVANCE(289);
      END_STATE();
    case 170:
      if (lookahead == 'u') ADVANCE(290);
      END_STATE();
    case 171:
      if (lookahead == 'e') ADVANCE(291);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(anon_sym_out);
      END_STATE();
    case 173:
      if (lookahead == 'k') ADVANCE(292);
      END_STATE();
    case 174:
      if (lookahead == 'a') ADVANCE(293);
      if (lookahead == 't') ADVANCE(294);
      END_STATE();
    case 175:
      if (lookahead == 'f') ADVANCE(295);
      END_STATE();
    case 176:
      if (lookahead == 't') ADVANCE(296);
      END_STATE();
    case 177:
      if (lookahead == 'd') ADVANCE(297);
      END_STATE();
    case 178:
      if (lookahead == 'v') ADVANCE(298);
      END_STATE();
    case 179:
      if (lookahead == 't') ADVANCE(299);
      END_STATE();
    case 180:
      if (lookahead == 'l') ADVANCE(300);
      END_STATE();
    case 181:
      if (lookahead == 'd') ADVANCE(301);
      END_STATE();
    case 182:
      if (lookahead == 'e') ADVANCE(302);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(anon_sym_ref);
      if (lookahead == 'e') ADVANCE(303);
      END_STATE();
    case 184:
      if (lookahead == 'd') ADVANCE(304);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_rep);
      END_STATE();
    case 186:
      if (lookahead == 'u') ADVANCE(305);
      END_STATE();
    case 187:
      if (lookahead == 'u') ADVANCE(306);
      END_STATE();
    case 188:
      if (lookahead == 'i') ADVANCE(307);
      END_STATE();
    case 189:
      if (lookahead == 'd') ADVANCE(308);
      END_STATE();
    case 190:
      if (lookahead == 'p') ADVANCE(309);
      END_STATE();
    case 191:
      if (lookahead == 'c') ADVANCE(310);
      END_STATE();
    case 192:
      if (lookahead == 'k') ADVANCE(311);
      if (lookahead == 'n') ADVANCE(312);
      if (lookahead == 't') ADVANCE(313);
      END_STATE();
    case 193:
      if (lookahead == 'p') ADVANCE(314);
      END_STATE();
    case 194:
      if (lookahead == 'u') ADVANCE(315);
      END_STATE();
    case 195:
      if (lookahead == 'c') ADVANCE(316);
      if (lookahead == 'j') ADVANCE(317);
      if (lookahead == 's') ADVANCE(318);
      if (lookahead == 't') ADVANCE(319);
      END_STATE();
    case 196:
      if (lookahead == 'c') ADVANCE(320);
      END_STATE();
    case 197:
      if (lookahead == 'm') ADVANCE(321);
      END_STATE();
    case 198:
      if (lookahead == 'n') ADVANCE(322);
      END_STATE();
    case 199:
      if (lookahead == 'e') ADVANCE(323);
      END_STATE();
    case 200:
      if (lookahead == 'n') ADVANCE(324);
      END_STATE();
    case 201:
      if (lookahead == 'e') ADVANCE(325);
      END_STATE();
    case 202:
      if (lookahead == 'e') ADVANCE(326);
      if (lookahead == 'i') ADVANCE(327);
      END_STATE();
    case 203:
      if (lookahead == 'o') ADVANCE(328);
      END_STATE();
    case 204:
      if (lookahead == 'i') ADVANCE(329);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(anon_sym_var);
      if (lookahead == 'i') ADVANCE(330);
      END_STATE();
    case 207:
      if (lookahead == 'i') ADVANCE(331);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(anon_sym_via);
      END_STATE();
    case 209:
      if (lookahead == 'w') ADVANCE(332);
      END_STATE();
    case 210:
      if (lookahead == 'n') ADVANCE(333);
      END_STATE();
    case 211:
      if (lookahead == 'l') ADVANCE(334);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(anon_sym_xor);
      END_STATE();
    case 213:
      if (lookahead == 't') ADVANCE(335);
      END_STATE();
    case 214:
      if (lookahead == 'r') ADVANCE(336);
      END_STATE();
    case 215:
      if (lookahead == 'p') ADVANCE(337);
      END_STATE();
    case 216:
      if (lookahead == 'o') ADVANCE(338);
      END_STATE();
    case 217:
      if (lookahead == 'r') ADVANCE(339);
      END_STATE();
    case 218:
      if (lookahead == 'r') ADVANCE(340);
      END_STATE();
    case 219:
      if (lookahead == 's') ADVANCE(341);
      END_STATE();
    case 220:
      if (lookahead == 'c') ADVANCE(342);
      END_STATE();
    case 221:
      if (lookahead == 'y') ADVANCE(343);
      END_STATE();
    case 222:
      if (lookahead == 'r') ADVANCE(344);
      END_STATE();
    case 223:
      if (lookahead == 'g') ADVANCE(345);
      END_STATE();
    case 224:
      if (lookahead == 'c') ADVANCE(346);
      END_STATE();
    case 225:
      if (lookahead == 'm') ADVANCE(347);
      END_STATE();
    case 226:
      if (lookahead == 'i') ADVANCE(348);
      END_STATE();
    case 227:
      if (lookahead == 'v') ADVANCE(349);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(anon_sym_bind);
      if (lookahead == 'i') ADVANCE(350);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(anon_sym_bool);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(anon_sym_calc);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(anon_sym_case);
      END_STATE();
    case 232:
      if (lookahead == 'n') ADVANCE(351);
      END_STATE();
    case 233:
      if (lookahead == 's') ADVANCE(352);
      END_STATE();
    case 234:
      if (lookahead == 'e') ADVANCE(353);
      END_STATE();
    case 235:
      if (lookahead == 'o') ADVANCE(354);
      END_STATE();
    case 236:
      if (lookahead == 'e') ADVANCE(355);
      END_STATE();
    case 237:
      if (lookahead == 'u') ADVANCE(356);
      END_STATE();
    case 238:
      if (lookahead == 'e') ADVANCE(357);
      END_STATE();
    case 239:
      if (lookahead == 't') ADVANCE(358);
      END_STATE();
    case 240:
      if (lookahead == 's') ADVANCE(359);
      END_STATE();
    case 241:
      if (lookahead == 't') ADVANCE(360);
      END_STATE();
    case 242:
      if (lookahead == 'd') ADVANCE(361);
      END_STATE();
    case 243:
      if (lookahead == 'u') ADVANCE(362);
      END_STATE();
    case 244:
      if (lookahead == 'n') ADVANCE(363);
      END_STATE();
    case 245:
      if (lookahead == 'n') ADVANCE(364);
      END_STATE();
    case 246:
      if (lookahead == 'v') ADVANCE(365);
      END_STATE();
    case 247:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 248:
      if (lookahead == 'o') ADVANCE(367);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 250:
      if (lookahead == 'y') ADVANCE(368);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(anon_sym_enum);
      END_STATE();
    case 252:
      if (lookahead == 't') ADVANCE(369);
      END_STATE();
    case 253:
      if (lookahead == 'b') ADVANCE(370);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(anon_sym_exit);
      END_STATE();
    case 255:
      if (lookahead == 's') ADVANCE(371);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(anon_sym_expr);
      END_STATE();
    case 257:
      if (lookahead == 'e') ADVANCE(372);
      END_STATE();
    case 258:
      if (lookahead == 'u') ADVANCE(373);
      END_STATE();
    case 259:
      if (lookahead == 'e') ADVANCE(374);
      END_STATE();
    case 260:
      if (lookahead == 't') ADVANCE(375);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_flow);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(anon_sym_fork);
      END_STATE();
    case 263:
      if (lookahead == 'e') ADVANCE(376);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(anon_sym_from);
      END_STATE();
    case 265:
      if (lookahead == 't') ADVANCE(377);
      END_STATE();
    case 266:
      if (lookahead == 'y') ADVANCE(378);
      END_STATE();
    case 267:
      if (lookahead == 'i') ADVANCE(379);
      END_STATE();
    case 268:
      if (lookahead == 'r') ADVANCE(380);
      END_STATE();
    case 269:
      if (lookahead == 'u') ADVANCE(381);
      END_STATE();
    case 270:
      if (lookahead == 'v') ADVANCE(382);
      END_STATE();
    case 271:
      if (lookahead == 't') ADVANCE(383);
      END_STATE();
    case 272:
      if (lookahead == 'r') ADVANCE(384);
      END_STATE();
    case 273:
      if (lookahead == 'r') ADVANCE(385);
      END_STATE();
    case 274:
      if (lookahead == 'p') ADVANCE(386);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(anon_sym_item);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(anon_sym_join);
      END_STATE();
    case 277:
      if (lookahead == 'u') ADVANCE(387);
      END_STATE();
    case 278:
      if (lookahead == 'a') ADVANCE(388);
      END_STATE();
    case 279:
      if (lookahead == 'l') ADVANCE(389);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_loop);
      END_STATE();
    case 281:
      if (lookahead == 'e') ADVANCE(390);
      END_STATE();
    case 282:
      if (lookahead == 'e') ADVANCE(391);
      END_STATE();
    case 283:
      if (lookahead == 'a') ADVANCE(392);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_meta);
      if (lookahead == 'c') ADVANCE(393);
      if (lookahead == 'd') ADVANCE(394);
      END_STATE();
    case 285:
      if (lookahead == 'i') ADVANCE(395);
      END_STATE();
    case 286:
      if (lookahead == 's') ADVANCE(396);
      END_STATE();
    case 287:
      if (lookahead == 'n') ADVANCE(397);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_null);
      END_STATE();
    case 289:
      if (lookahead == 'c') ADVANCE(398);
      END_STATE();
    case 290:
      if (lookahead == 'r') ADVANCE(399);
      END_STATE();
    case 291:
      if (lookahead == 'r') ADVANCE(400);
      END_STATE();
    case 292:
      if (lookahead == 'a') ADVANCE(401);
      END_STATE();
    case 293:
      if (lookahead == 'l') ADVANCE(402);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(anon_sym_part);
      END_STATE();
    case 295:
      if (lookahead == 'o') ADVANCE(403);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_port);
      if (lookahead == 'i') ADVANCE(404);
      END_STATE();
    case 297:
      if (lookahead == 'i') ADVANCE(405);
      END_STATE();
    case 298:
      if (lookahead == 'a') ADVANCE(406);
      END_STATE();
    case 299:
      if (lookahead == 'e') ADVANCE(407);
      END_STATE();
    case 300:
      if (lookahead == 'i') ADVANCE(408);
      END_STATE();
    case 301:
      if (lookahead == 'o') ADVANCE(409);
      END_STATE();
    case 302:
      if (lookahead == 'f') ADVANCE(410);
      END_STATE();
    case 303:
      if (lookahead == 'r') ADVANCE(411);
      END_STATE();
    case 304:
      if (lookahead == 'e') ADVANCE(412);
      END_STATE();
    case 305:
      if (lookahead == 'i') ADVANCE(413);
      END_STATE();
    case 306:
      if (lookahead == 'r') ADVANCE(414);
      END_STATE();
    case 307:
      if (lookahead == 's') ADVANCE(415);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_send);
      END_STATE();
    case 309:
      if (lookahead == 's') ADVANCE(416);
      END_STATE();
    case 310:
      if (lookahead == 'i') ADVANCE(417);
      END_STATE();
    case 311:
      if (lookahead == 'e') ADVANCE(418);
      END_STATE();
    case 312:
      if (lookahead == 'd') ADVANCE(419);
      END_STATE();
    case 313:
      if (lookahead == 'e') ADVANCE(420);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(anon_sym_step);
      END_STATE();
    case 315:
      if (lookahead == 'c') ADVANCE(421);
      END_STATE();
    case 316:
      if (lookahead == 'l') ADVANCE(422);
      END_STATE();
    case 317:
      if (lookahead == 'e') ADVANCE(423);
      END_STATE();
    case 318:
      if (lookahead == 'e') ADVANCE(424);
      END_STATE();
    case 319:
      if (lookahead == 'y') ADVANCE(425);
      END_STATE();
    case 320:
      if (lookahead == 'e') ADVANCE(426);
      END_STATE();
    case 321:
      if (lookahead == 'i') ADVANCE(427);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(anon_sym_then);
      END_STATE();
    case 323:
      if (lookahead == 's') ADVANCE(428);
      END_STATE();
    case 324:
      if (lookahead == 's') ADVANCE(429);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(430);
      END_STATE();
    case 327:
      if (lookahead == 'n') ADVANCE(431);
      END_STATE();
    case 328:
      if (lookahead == 'n') ADVANCE(432);
      END_STATE();
    case 329:
      if (lookahead == 'l') ADVANCE(433);
      END_STATE();
    case 330:
      if (lookahead == 'a') ADVANCE(434);
      END_STATE();
    case 331:
      if (lookahead == 'f') ADVANCE(435);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(anon_sym_view);
      if (lookahead == 'p') ADVANCE(436);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_when);
      END_STATE();
    case 334:
      if (lookahead == 'e') ADVANCE(437);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(anon_sym_about);
      END_STATE();
    case 336:
      if (lookahead == 'a') ADVANCE(438);
      END_STATE();
    case 337:
      if (lookahead == 't') ADVANCE(439);
      END_STATE();
    case 338:
      if (lookahead == 'n') ADVANCE(440);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_actor);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(anon_sym_after);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_alias);
      END_STATE();
    case 342:
      if (lookahead == 'a') ADVANCE(441);
      END_STATE();
    case 343:
      if (lookahead == 's') ADVANCE(442);
      END_STATE();
    case 344:
      if (lookahead == 't') ADVANCE(443);
      END_STATE();
    case 345:
      if (lookahead == 'n') ADVANCE(444);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(anon_sym_assoc);
      END_STATE();
    case 347:
      if (lookahead == 'e') ADVANCE(445);
      END_STATE();
    case 348:
      if (lookahead == 'b') ADVANCE(446);
      END_STATE();
    case 349:
      if (lookahead == 'i') ADVANCE(447);
      END_STATE();
    case 350:
      if (lookahead == 'n') ADVANCE(448);
      END_STATE();
    case 351:
      if (lookahead == 's') ADVANCE(449);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(anon_sym_class);
      if (lookahead == 'i') ADVANCE(450);
      END_STATE();
    case 353:
      if (lookahead == 'n') ADVANCE(451);
      END_STATE();
    case 354:
      if (lookahead == 's') ADVANCE(452);
      END_STATE();
    case 355:
      if (lookahead == 'r') ADVANCE(453);
      END_STATE();
    case 356:
      if (lookahead == 'g') ADVANCE(454);
      END_STATE();
    case 357:
      if (lookahead == 'c') ADVANCE(455);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_const);
      if (lookahead == 'a') ADVANCE(456);
      if (lookahead == 'r') ADVANCE(457);
      END_STATE();
    case 359:
      if (lookahead == 'e') ADVANCE(458);
      END_STATE();
    case 360:
      if (lookahead == 'y') ADVANCE(459);
      END_STATE();
    case 361:
      if (lookahead == 'e') ADVANCE(460);
      END_STATE();
    case 362:
      if (lookahead == 'l') ADVANCE(461);
      END_STATE();
    case 363:
      if (lookahead == 'e') ADVANCE(462);
      END_STATE();
    case 364:
      if (lookahead == 'd') ADVANCE(463);
      END_STATE();
    case 365:
      if (lookahead == 'e') ADVANCE(464);
      END_STATE();
    case 366:
      if (lookahead == 'r') ADVANCE(465);
      END_STATE();
    case 367:
      if (lookahead == 'i') ADVANCE(466);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_entry);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_event);
      END_STATE();
    case 370:
      if (lookahead == 'i') ADVANCE(467);
      END_STATE();
    case 371:
      if (lookahead == 'e') ADVANCE(468);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 373:
      if (lookahead == 'r') ADVANCE(469);
      END_STATE();
    case 374:
      if (lookahead == 'r') ADVANCE(470);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(anon_sym_first);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_frame);
      END_STATE();
    case 377:
      if (lookahead == 'i') ADVANCE(471);
      END_STATE();
    case 378:
      if (lookahead == 'p') ADVANCE(472);
      END_STATE();
    case 379:
      if (lookahead == 'e') ADVANCE(473);
      END_STATE();
    case 380:
      if (lookahead == 't') ADVANCE(474);
      END_STATE();
    case 381:
      if (lookahead == 'd') ADVANCE(475);
      END_STATE();
    case 382:
      if (lookahead == 'i') ADVANCE(476);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(anon_sym_inout);
      END_STATE();
    case 384:
      if (lookahead == 'a') ADVANCE(477);
      if (lookahead == 'f') ADVANCE(478);
      if (lookahead == 's') ADVANCE(479);
      END_STATE();
    case 385:
      if (lookahead == 's') ADVANCE(480);
      if (lookahead == 't') ADVANCE(481);
      END_STATE();
    case 386:
      if (lookahead == 'e') ADVANCE(482);
      END_STATE();
    case 387:
      if (lookahead == 'a') ADVANCE(483);
      END_STATE();
    case 388:
      if (lookahead == 'r') ADVANCE(484);
      END_STATE();
    case 389:
      if (lookahead == 'e') ADVANCE(485);
      END_STATE();
    case 390:
      if (lookahead == 'r') ADVANCE(486);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_merge);
      END_STATE();
    case 392:
      if (lookahead == 'g') ADVANCE(487);
      END_STATE();
    case 393:
      if (lookahead == 'l') ADVANCE(488);
      END_STATE();
    case 394:
      if (lookahead == 'a') ADVANCE(489);
      END_STATE();
    case 395:
      if (lookahead == 'p') ADVANCE(490);
      END_STATE();
    case 396:
      if (lookahead == 'p') ADVANCE(491);
      END_STATE();
    case 397:
      if (lookahead == 'i') ADVANCE(492);
      END_STATE();
    case 398:
      if (lookahead == 't') ADVANCE(493);
      END_STATE();
    case 399:
      if (lookahead == 'r') ADVANCE(494);
      END_STATE();
    case 400:
      if (lookahead == 'e') ADVANCE(495);
      END_STATE();
    case 401:
      if (lookahead == 'g') ADVANCE(496);
      END_STATE();
    case 402:
      if (lookahead == 'l') ADVANCE(497);
      END_STATE();
    case 403:
      if (lookahead == 'r') ADVANCE(498);
      END_STATE();
    case 404:
      if (lookahead == 'o') ADVANCE(499);
      END_STATE();
    case 405:
      if (lookahead == 'c') ADVANCE(500);
      END_STATE();
    case 406:
      if (lookahead == 't') ADVANCE(501);
      END_STATE();
    case 407:
      if (lookahead == 'c') ADVANCE(502);
      END_STATE();
    case 408:
      if (lookahead == 'c') ADVANCE(503);
      END_STATE();
    case 409:
      if (lookahead == 'n') ADVANCE(504);
      END_STATE();
    case 410:
      if (lookahead == 'i') ADVANCE(505);
      END_STATE();
    case 411:
      if (lookahead == 'e') ADVANCE(506);
      END_STATE();
    case 412:
      if (lookahead == 'r') ADVANCE(507);
      END_STATE();
    case 413:
      if (lookahead == 'r') ADVANCE(508);
      END_STATE();
    case 414:
      if (lookahead == 'n') ADVANCE(509);
      END_STATE();
    case 415:
      if (lookahead == 'f') ADVANCE(510);
      END_STATE();
    case 416:
      if (lookahead == 'h') ADVANCE(511);
      END_STATE();
    case 417:
      if (lookahead == 'a') ADVANCE(512);
      END_STATE();
    case 418:
      if (lookahead == 'h') ADVANCE(513);
      END_STATE();
    case 419:
      if (lookahead == 'a') ADVANCE(514);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 421:
      if (lookahead == 't') ADVANCE(515);
      END_STATE();
    case 422:
      if (lookahead == 'a') ADVANCE(516);
      END_STATE();
    case 423:
      if (lookahead == 'c') ADVANCE(517);
      END_STATE();
    case 424:
      if (lookahead == 't') ADVANCE(518);
      END_STATE();
    case 425:
      if (lookahead == 'p') ADVANCE(519);
      END_STATE();
    case 426:
      if (lookahead == 's') ADVANCE(520);
      END_STATE();
    case 427:
      if (lookahead == 'n') ADVANCE(521);
      END_STATE();
    case 428:
      if (lookahead == 'l') ADVANCE(522);
      END_STATE();
    case 429:
      if (lookahead == 'i') ADVANCE(523);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(anon_sym_typed);
      END_STATE();
    case 431:
      if (lookahead == 'g') ADVANCE(524);
      END_STATE();
    case 432:
      if (lookahead == 's') ADVANCE(525);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(anon_sym_until);
      END_STATE();
    case 434:
      if (lookahead == 'n') ADVANCE(526);
      if (lookahead == 't') ADVANCE(527);
      END_STATE();
    case 435:
      if (lookahead == 'i') ADVANCE(528);
      if (lookahead == 'y') ADVANCE(529);
      END_STATE();
    case 436:
      if (lookahead == 'o') ADVANCE(530);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_while);
      END_STATE();
    case 438:
      if (lookahead == 'c') ADVANCE(531);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_accept);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(anon_sym_action);
      END_STATE();
    case 441:
      if (lookahead == 't') ADVANCE(532);
      END_STATE();
    case 442:
      if (lookahead == 'i') ADVANCE(533);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(anon_sym_assert);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_assign);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(anon_sym_assume);
      END_STATE();
    case 446:
      if (lookahead == 'u') ADVANCE(534);
      END_STATE();
    case 447:
      if (lookahead == 'o') ADVANCE(535);
      END_STATE();
    case 448:
      if (lookahead == 'g') ADVANCE(536);
      END_STATE();
    case 449:
      ACCEPT_TOKEN(anon_sym_chains);
      END_STATE();
    case 450:
      if (lookahead == 'f') ADVANCE(537);
      END_STATE();
    case 451:
      if (lookahead == 't') ADVANCE(538);
      END_STATE();
    case 452:
      if (lookahead == 'i') ADVANCE(539);
      END_STATE();
    case 453:
      if (lookahead == 'n') ADVANCE(540);
      END_STATE();
    case 454:
      if (lookahead == 'a') ADVANCE(541);
      END_STATE();
    case 455:
      if (lookahead == 't') ADVANCE(542);
      END_STATE();
    case 456:
      if (lookahead == 'n') ADVANCE(543);
      END_STATE();
    case 457:
      if (lookahead == 'a') ADVANCE(544);
      END_STATE();
    case 458:
      if (lookahead == 's') ADVANCE(545);
      END_STATE();
    case 459:
      if (lookahead == 'p') ADVANCE(546);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(anon_sym_decide);
      END_STATE();
    case 461:
      if (lookahead == 't') ADVANCE(547);
      END_STATE();
    case 462:
      if (lookahead == 'd') ADVANCE(548);
      END_STATE();
    case 463:
      if (lookahead == 'e') ADVANCE(549);
      END_STATE();
    case 464:
      if (lookahead == 'd') ADVANCE(550);
      END_STATE();
    case 465:
      if (lookahead == 'e') ADVANCE(551);
      END_STATE();
    case 466:
      if (lookahead == 'n') ADVANCE(552);
      END_STATE();
    case 467:
      if (lookahead == 't') ADVANCE(553);
      END_STATE();
    case 468:
      ACCEPT_TOKEN(anon_sym_expose);
      END_STATE();
    case 469:
      if (lookahead == 'e') ADVANCE(554);
      if (lookahead == 'i') ADVANCE(555);
      END_STATE();
    case 470:
      ACCEPT_TOKEN(anon_sym_filter);
      END_STATE();
    case 471:
      if (lookahead == 'o') ADVANCE(556);
      END_STATE();
    case 472:
      if (lookahead == 'e') ADVANCE(557);
      END_STATE();
    case 473:
      if (lookahead == 's') ADVANCE(558);
      END_STATE();
    case 474:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 475:
      if (lookahead == 'e') ADVANCE(559);
      END_STATE();
    case 476:
      if (lookahead == 'd') ADVANCE(560);
      END_STATE();
    case 477:
      if (lookahead == 'c') ADVANCE(561);
      END_STATE();
    case 478:
      if (lookahead == 'a') ADVANCE(562);
      END_STATE();
    case 479:
      if (lookahead == 'e') ADVANCE(563);
      END_STATE();
    case 480:
      if (lookahead == 'e') ADVANCE(564);
      END_STATE();
    case 481:
      if (lookahead == 'i') ADVANCE(565);
      END_STATE();
    case 482:
      ACCEPT_TOKEN(anon_sym_istype);
      END_STATE();
    case 483:
      if (lookahead == 'g') ADVANCE(566);
      END_STATE();
    case 484:
      if (lookahead == 'y') ADVANCE(567);
      END_STATE();
    case 485:
      ACCEPT_TOKEN(anon_sym_locale);
      END_STATE();
    case 486:
      ACCEPT_TOKEN(anon_sym_member);
      END_STATE();
    case 487:
      if (lookahead == 'e') ADVANCE(568);
      END_STATE();
    case 488:
      if (lookahead == 'a') ADVANCE(569);
      END_STATE();
    case 489:
      if (lookahead == 't') ADVANCE(570);
      END_STATE();
    case 490:
      if (lookahead == 'l') ADVANCE(571);
      END_STATE();
    case 491:
      if (lookahead == 'a') ADVANCE(572);
      END_STATE();
    case 492:
      if (lookahead == 'q') ADVANCE(573);
      END_STATE();
    case 493:
      if (lookahead == 'i') ADVANCE(574);
      END_STATE();
    case 494:
      if (lookahead == 'e') ADVANCE(575);
      END_STATE();
    case 495:
      if (lookahead == 'd') ADVANCE(576);
      END_STATE();
    case 496:
      if (lookahead == 'e') ADVANCE(577);
      END_STATE();
    case 497:
      if (lookahead == 'e') ADVANCE(578);
      END_STATE();
    case 498:
      if (lookahead == 'm') ADVANCE(579);
      END_STATE();
    case 499:
      if (lookahead == 'n') ADVANCE(580);
      END_STATE();
    case 500:
      if (lookahead == 'a') ADVANCE(581);
      END_STATE();
    case 501:
      if (lookahead == 'e') ADVANCE(582);
      END_STATE();
    case 502:
      if (lookahead == 't') ADVANCE(583);
      END_STATE();
    case 503:
      ACCEPT_TOKEN(anon_sym_public);
      END_STATE();
    case 504:
      if (lookahead == 'l') ADVANCE(584);
      END_STATE();
    case 505:
      if (lookahead == 'n') ADVANCE(585);
      END_STATE();
    case 506:
      if (lookahead == 'n') ADVANCE(586);
      END_STATE();
    case 507:
      ACCEPT_TOKEN(anon_sym_render);
      if (lookahead == 'i') ADVANCE(587);
      END_STATE();
    case 508:
      if (lookahead == 'e') ADVANCE(588);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(anon_sym_return);
      END_STATE();
    case 510:
      if (lookahead == 'y') ADVANCE(589);
      END_STATE();
    case 511:
      if (lookahead == 'o') ADVANCE(590);
      END_STATE();
    case 512:
      if (lookahead == 'l') ADVANCE(591);
      END_STATE();
    case 513:
      if (lookahead == 'o') ADVANCE(592);
      END_STATE();
    case 514:
      if (lookahead == 'r') ADVANCE(593);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 516:
      if (lookahead == 's') ADVANCE(594);
      END_STATE();
    case 517:
      if (lookahead == 't') ADVANCE(595);
      END_STATE();
    case 518:
      ACCEPT_TOKEN(anon_sym_subset);
      if (lookahead == 's') ADVANCE(596);
      END_STATE();
    case 519:
      if (lookahead == 'e') ADVANCE(597);
      END_STATE();
    case 520:
      if (lookahead == 's') ADVANCE(598);
      END_STATE();
    case 521:
      if (lookahead == 'a') ADVANCE(599);
      END_STATE();
    case 522:
      if (lookahead == 'i') ADVANCE(600);
      END_STATE();
    case 523:
      if (lookahead == 't') ADVANCE(601);
      END_STATE();
    case 524:
      ACCEPT_TOKEN(anon_sym_typing);
      END_STATE();
    case 525:
      ACCEPT_TOKEN(anon_sym_unions);
      END_STATE();
    case 526:
      if (lookahead == 't') ADVANCE(602);
      END_STATE();
    case 527:
      if (lookahead == 'i') ADVANCE(603);
      END_STATE();
    case 528:
      if (lookahead == 'c') ADVANCE(604);
      END_STATE();
    case 529:
      ACCEPT_TOKEN(anon_sym_verify);
      END_STATE();
    case 530:
      if (lookahead == 'i') ADVANCE(605);
      END_STATE();
    case 531:
      if (lookahead == 't') ADVANCE(606);
      END_STATE();
    case 532:
      if (lookahead == 'e') ADVANCE(607);
      if (lookahead == 'i') ADVANCE(608);
      END_STATE();
    case 533:
      if (lookahead == 's') ADVANCE(609);
      END_STATE();
    case 534:
      if (lookahead == 't') ADVANCE(610);
      END_STATE();
    case 535:
      if (lookahead == 'r') ADVANCE(611);
      END_STATE();
    case 536:
      ACCEPT_TOKEN(anon_sym_binding);
      END_STATE();
    case 537:
      if (lookahead == 'i') ADVANCE(612);
      END_STATE();
    case 538:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 539:
      if (lookahead == 't') ADVANCE(613);
      END_STATE();
    case 540:
      ACCEPT_TOKEN(anon_sym_concern);
      END_STATE();
    case 541:
      if (lookahead == 't') ADVANCE(614);
      END_STATE();
    case 542:
      ACCEPT_TOKEN(anon_sym_connect);
      if (lookahead == 'i') ADVANCE(615);
      if (lookahead == 'o') ADVANCE(616);
      END_STATE();
    case 543:
      if (lookahead == 't') ADVANCE(617);
      END_STATE();
    case 544:
      if (lookahead == 'i') ADVANCE(618);
      END_STATE();
    case 545:
      ACCEPT_TOKEN(anon_sym_crosses);
      END_STATE();
    case 546:
      if (lookahead == 'e') ADVANCE(619);
      END_STATE();
    case 547:
      ACCEPT_TOKEN(anon_sym_default);
      END_STATE();
    case 548:
      ACCEPT_TOKEN(anon_sym_defined);
      END_STATE();
    case 549:
      if (lookahead == 'n') ADVANCE(620);
      END_STATE();
    case 550:
      ACCEPT_TOKEN(anon_sym_derived);
      END_STATE();
    case 551:
      if (lookahead == 'n') ADVANCE(621);
      END_STATE();
    case 552:
      if (lookahead == 'i') ADVANCE(622);
      if (lookahead == 't') ADVANCE(623);
      END_STATE();
    case 553:
      ACCEPT_TOKEN(anon_sym_exhibit);
      END_STATE();
    case 554:
      ACCEPT_TOKEN(anon_sym_feature);
      if (lookahead == 'd') ADVANCE(624);
      END_STATE();
    case 555:
      if (lookahead == 'n') ADVANCE(625);
      END_STATE();
    case 556:
      if (lookahead == 'n') ADVANCE(626);
      END_STATE();
    case 557:
      ACCEPT_TOKEN(anon_sym_hastype);
      END_STATE();
    case 558:
      ACCEPT_TOKEN(anon_sym_implies);
      END_STATE();
    case 559:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 560:
      if (lookahead == 'u') ADVANCE(627);
      END_STATE();
    case 561:
      if (lookahead == 't') ADVANCE(628);
      END_STATE();
    case 562:
      if (lookahead == 'c') ADVANCE(629);
      END_STATE();
    case 563:
      if (lookahead == 'c') ADVANCE(630);
      END_STATE();
    case 564:
      ACCEPT_TOKEN(anon_sym_inverse);
      END_STATE();
    case 565:
      if (lookahead == 'n') ADVANCE(631);
      END_STATE();
    case 566:
      if (lookahead == 'e') ADVANCE(632);
      END_STATE();
    case 567:
      ACCEPT_TOKEN(anon_sym_library);
      END_STATE();
    case 568:
      ACCEPT_TOKEN(anon_sym_message);
      END_STATE();
    case 569:
      if (lookahead == 's') ADVANCE(633);
      END_STATE();
    case 570:
      if (lookahead == 'a') ADVANCE(634);
      END_STATE();
    case 571:
      if (lookahead == 'i') ADVANCE(635);
      END_STATE();
    case 572:
      if (lookahead == 'c') ADVANCE(636);
      END_STATE();
    case 573:
      if (lookahead == 'u') ADVANCE(637);
      END_STATE();
    case 574:
      if (lookahead == 'v') ADVANCE(638);
      END_STATE();
    case 575:
      if (lookahead == 'n') ADVANCE(639);
      END_STATE();
    case 576:
      ACCEPT_TOKEN(anon_sym_ordered);
      END_STATE();
    case 577:
      ACCEPT_TOKEN(anon_sym_package);
      END_STATE();
    case 578:
      if (lookahead == 'l') ADVANCE(640);
      END_STATE();
    case 579:
      ACCEPT_TOKEN(anon_sym_perform);
      END_STATE();
    case 580:
      ACCEPT_TOKEN(anon_sym_portion);
      END_STATE();
    case 581:
      if (lookahead == 't') ADVANCE(641);
      END_STATE();
    case 582:
      ACCEPT_TOKEN(anon_sym_private);
      END_STATE();
    case 583:
      if (lookahead == 'e') ADVANCE(642);
      END_STATE();
    case 584:
      if (lookahead == 'y') ADVANCE(643);
      END_STATE();
    case 585:
      if (lookahead == 'e') ADVANCE(644);
      if (lookahead == 'i') ADVANCE(645);
      END_STATE();
    case 586:
      if (lookahead == 'c') ADVANCE(646);
      END_STATE();
    case 587:
      if (lookahead == 'n') ADVANCE(647);
      END_STATE();
    case 588:
      ACCEPT_TOKEN(anon_sym_require);
      if (lookahead == 'm') ADVANCE(648);
      END_STATE();
    case 589:
      ACCEPT_TOKEN(anon_sym_satisfy);
      END_STATE();
    case 590:
      if (lookahead == 't') ADVANCE(649);
      END_STATE();
    case 591:
      if (lookahead == 'i') ADVANCE(650);
      END_STATE();
    case 592:
      if (lookahead == 'l') ADVANCE(651);
      END_STATE();
    case 593:
      if (lookahead == 'd') ADVANCE(652);
      END_STATE();
    case 594:
      if (lookahead == 's') ADVANCE(653);
      END_STATE();
    case 595:
      ACCEPT_TOKEN(anon_sym_subject);
      END_STATE();
    case 596:
      ACCEPT_TOKEN(anon_sym_subsets);
      END_STATE();
    case 597:
      ACCEPT_TOKEN(anon_sym_subtype);
      END_STATE();
    case 598:
      if (lookahead == 'i') ADVANCE(654);
      END_STATE();
    case 599:
      if (lookahead == 't') ADVANCE(655);
      END_STATE();
    case 600:
      if (lookahead == 'c') ADVANCE(656);
      END_STATE();
    case 601:
      if (lookahead == 'i') ADVANCE(657);
      END_STATE();
    case 602:
      ACCEPT_TOKEN(anon_sym_variant);
      END_STATE();
    case 603:
      if (lookahead == 'o') ADVANCE(658);
      END_STATE();
    case 604:
      if (lookahead == 'a') ADVANCE(659);
      END_STATE();
    case 605:
      if (lookahead == 'n') ADVANCE(660);
      END_STATE();
    case 606:
      ACCEPT_TOKEN(anon_sym_abstract);
      END_STATE();
    case 607:
      ACCEPT_TOKEN(anon_sym_allocate);
      END_STATE();
    case 608:
      if (lookahead == 'o') ADVANCE(661);
      END_STATE();
    case 609:
      ACCEPT_TOKEN(anon_sym_analysis);
      END_STATE();
    case 610:
      if (lookahead == 'e') ADVANCE(662);
      END_STATE();
    case 611:
      ACCEPT_TOKEN(anon_sym_behavior);
      END_STATE();
    case 612:
      if (lookahead == 'e') ADVANCE(663);
      END_STATE();
    case 613:
      if (lookahead == 'e') ADVANCE(664);
      END_STATE();
    case 614:
      if (lookahead == 'e') ADVANCE(665);
      if (lookahead == 'i') ADVANCE(666);
      END_STATE();
    case 615:
      if (lookahead == 'o') ADVANCE(667);
      END_STATE();
    case 616:
      if (lookahead == 'r') ADVANCE(668);
      END_STATE();
    case 617:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 618:
      if (lookahead == 'n') ADVANCE(669);
      END_STATE();
    case 619:
      ACCEPT_TOKEN(anon_sym_datatype);
      END_STATE();
    case 620:
      if (lookahead == 'c') ADVANCE(670);
      END_STATE();
    case 621:
      if (lookahead == 'c') ADVANCE(671);
      END_STATE();
    case 622:
      if (lookahead == 'n') ADVANCE(672);
      END_STATE();
    case 623:
      ACCEPT_TOKEN(anon_sym_disjoint);
      END_STATE();
    case 624:
      ACCEPT_TOKEN(anon_sym_featured);
      END_STATE();
    case 625:
      if (lookahead == 'g') ADVANCE(673);
      END_STATE();
    case 626:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 627:
      if (lookahead == 'a') ADVANCE(674);
      END_STATE();
    case 628:
      if (lookahead == 'i') ADVANCE(675);
      END_STATE();
    case 629:
      if (lookahead == 'e') ADVANCE(676);
      END_STATE();
    case 630:
      if (lookahead == 't') ADVANCE(677);
      END_STATE();
    case 631:
      if (lookahead == 'g') ADVANCE(678);
      END_STATE();
    case 632:
      ACCEPT_TOKEN(anon_sym_language);
      END_STATE();
    case 633:
      if (lookahead == 's') ADVANCE(679);
      END_STATE();
    case 634:
      ACCEPT_TOKEN(anon_sym_metadata);
      END_STATE();
    case 635:
      if (lookahead == 'c') ADVANCE(680);
      END_STATE();
    case 636:
      if (lookahead == 'e') ADVANCE(681);
      END_STATE();
    case 637:
      if (lookahead == 'e') ADVANCE(682);
      END_STATE();
    case 638:
      if (lookahead == 'e') ADVANCE(683);
      END_STATE();
    case 639:
      if (lookahead == 'c') ADVANCE(684);
      END_STATE();
    case 640:
      ACCEPT_TOKEN(anon_sym_parallel);
      END_STATE();
    case 641:
      if (lookahead == 'e') ADVANCE(685);
      END_STATE();
    case 642:
      if (lookahead == 'd') ADVANCE(686);
      END_STATE();
    case 643:
      ACCEPT_TOKEN(anon_sym_readonly);
      END_STATE();
    case 644:
      if (lookahead == 's') ADVANCE(687);
      END_STATE();
    case 645:
      if (lookahead == 't') ADVANCE(688);
      END_STATE();
    case 646:
      if (lookahead == 'e') ADVANCE(689);
      END_STATE();
    case 647:
      if (lookahead == 'g') ADVANCE(690);
      END_STATE();
    case 648:
      if (lookahead == 'e') ADVANCE(691);
      END_STATE();
    case 649:
      ACCEPT_TOKEN(anon_sym_snapshot);
      END_STATE();
    case 650:
      if (lookahead == 'z') ADVANCE(692);
      END_STATE();
    case 651:
      if (lookahead == 'd') ADVANCE(693);
      END_STATE();
    case 652:
      ACCEPT_TOKEN(anon_sym_standard);
      END_STATE();
    case 653:
      if (lookahead == 'i') ADVANCE(694);
      END_STATE();
    case 654:
      if (lookahead == 'o') ADVANCE(695);
      END_STATE();
    case 655:
      if (lookahead == 'e') ADVANCE(696);
      END_STATE();
    case 656:
      if (lookahead == 'e') ADVANCE(697);
      END_STATE();
    case 657:
      if (lookahead == 'o') ADVANCE(698);
      END_STATE();
    case 658:
      if (lookahead == 'n') ADVANCE(699);
      END_STATE();
    case 659:
      if (lookahead == 't') ADVANCE(700);
      END_STATE();
    case 660:
      if (lookahead == 't') ADVANCE(701);
      END_STATE();
    case 661:
      if (lookahead == 'n') ADVANCE(702);
      END_STATE();
    case 662:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 663:
      if (lookahead == 'r') ADVANCE(703);
      END_STATE();
    case 664:
      ACCEPT_TOKEN(anon_sym_composite);
      END_STATE();
    case 665:
      ACCEPT_TOKEN(anon_sym_conjugate);
      if (lookahead == 's') ADVANCE(704);
      END_STATE();
    case 666:
      if (lookahead == 'o') ADVANCE(705);
      END_STATE();
    case 667:
      if (lookahead == 'n') ADVANCE(706);
      END_STATE();
    case 668:
      ACCEPT_TOKEN(anon_sym_connector);
      END_STATE();
    case 669:
      if (lookahead == 't') ADVANCE(707);
      END_STATE();
    case 670:
      if (lookahead == 'y') ADVANCE(708);
      END_STATE();
    case 671:
      if (lookahead == 'e') ADVANCE(709);
      END_STATE();
    case 672:
      if (lookahead == 'g') ADVANCE(710);
      END_STATE();
    case 673:
      ACCEPT_TOKEN(anon_sym_featuring);
      END_STATE();
    case 674:
      if (lookahead == 'l') ADVANCE(711);
      END_STATE();
    case 675:
      if (lookahead == 'o') ADVANCE(712);
      END_STATE();
    case 676:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 677:
      if (lookahead == 's') ADVANCE(713);
      END_STATE();
    case 678:
      ACCEPT_TOKEN(anon_sym_inverting);
      END_STATE();
    case 679:
      ACCEPT_TOKEN(anon_sym_metaclass);
      END_STATE();
    case 680:
      if (lookahead == 'i') ADVANCE(714);
      END_STATE();
    case 681:
      ACCEPT_TOKEN(anon_sym_namespace);
      END_STATE();
    case 682:
      ACCEPT_TOKEN(anon_sym_nonunique);
      END_STATE();
    case 683:
      ACCEPT_TOKEN(anon_sym_objective);
      END_STATE();
    case 684:
      if (lookahead == 'e') ADVANCE(715);
      END_STATE();
    case 685:
      ACCEPT_TOKEN(anon_sym_predicate);
      END_STATE();
    case 686:
      ACCEPT_TOKEN(anon_sym_protected);
      END_STATE();
    case 687:
      ACCEPT_TOKEN(anon_sym_redefines);
      END_STATE();
    case 688:
      if (lookahead == 'i') ADVANCE(716);
      END_STATE();
    case 689:
      if (lookahead == 's') ADVANCE(717);
      END_STATE();
    case 690:
      ACCEPT_TOKEN(anon_sym_rendering);
      END_STATE();
    case 691:
      if (lookahead == 'n') ADVANCE(718);
      END_STATE();
    case 692:
      if (lookahead == 'a') ADVANCE(719);
      if (lookahead == 'e') ADVANCE(720);
      END_STATE();
    case 693:
      if (lookahead == 'e') ADVANCE(721);
      END_STATE();
    case 694:
      if (lookahead == 'f') ADVANCE(722);
      END_STATE();
    case 695:
      if (lookahead == 'n') ADVANCE(723);
      END_STATE();
    case 696:
      ACCEPT_TOKEN(anon_sym_terminate);
      END_STATE();
    case 697:
      ACCEPT_TOKEN(anon_sym_timeslice);
      END_STATE();
    case 698:
      if (lookahead == 'n') ADVANCE(724);
      END_STATE();
    case 699:
      ACCEPT_TOKEN(anon_sym_variation);
      END_STATE();
    case 700:
      if (lookahead == 'i') ADVANCE(725);
      END_STATE();
    case 701:
      ACCEPT_TOKEN(anon_sym_viewpoint);
      END_STATE();
    case 702:
      ACCEPT_TOKEN(anon_sym_allocation);
      END_STATE();
    case 703:
      ACCEPT_TOKEN(anon_sym_classifier);
      END_STATE();
    case 704:
      ACCEPT_TOKEN(anon_sym_conjugates);
      END_STATE();
    case 705:
      if (lookahead == 'n') ADVANCE(726);
      END_STATE();
    case 706:
      ACCEPT_TOKEN(anon_sym_connection);
      END_STATE();
    case 707:
      ACCEPT_TOKEN(anon_sym_constraint);
      END_STATE();
    case 708:
      ACCEPT_TOKEN(anon_sym_dependency);
      END_STATE();
    case 709:
      if (lookahead == 's') ADVANCE(727);
      END_STATE();
    case 710:
      ACCEPT_TOKEN(anon_sym_disjoining);
      END_STATE();
    case 711:
      ACCEPT_TOKEN(anon_sym_individual);
      END_STATE();
    case 712:
      if (lookahead == 'n') ADVANCE(728);
      END_STATE();
    case 713:
      ACCEPT_TOKEN(anon_sym_intersects);
      END_STATE();
    case 714:
      if (lookahead == 't') ADVANCE(729);
      END_STATE();
    case 715:
      ACCEPT_TOKEN(anon_sym_occurrence);
      END_STATE();
    case 716:
      if (lookahead == 'o') ADVANCE(730);
      END_STATE();
    case 717:
      ACCEPT_TOKEN(anon_sym_references);
      END_STATE();
    case 718:
      if (lookahead == 't') ADVANCE(731);
      END_STATE();
    case 719:
      if (lookahead == 't') ADVANCE(732);
      END_STATE();
    case 720:
      if (lookahead == 's') ADVANCE(733);
      END_STATE();
    case 721:
      if (lookahead == 'r') ADVANCE(734);
      END_STATE();
    case 722:
      if (lookahead == 'i') ADVANCE(735);
      END_STATE();
    case 723:
      ACCEPT_TOKEN(anon_sym_succession);
      END_STATE();
    case 724:
      ACCEPT_TOKEN(anon_sym_transition);
      END_STATE();
    case 725:
      if (lookahead == 'o') ADVANCE(736);
      END_STATE();
    case 726:
      ACCEPT_TOKEN(anon_sym_conjugation);
      END_STATE();
    case 727:
      ACCEPT_TOKEN(anon_sym_differences);
      END_STATE();
    case 728:
      ACCEPT_TOKEN(anon_sym_interaction);
      END_STATE();
    case 729:
      if (lookahead == 'y') ADVANCE(737);
      END_STATE();
    case 730:
      if (lookahead == 'n') ADVANCE(738);
      END_STATE();
    case 731:
      ACCEPT_TOKEN(anon_sym_requirement);
      END_STATE();
    case 732:
      if (lookahead == 'i') ADVANCE(739);
      END_STATE();
    case 733:
      ACCEPT_TOKEN(anon_sym_specializes);
      END_STATE();
    case 734:
      ACCEPT_TOKEN(anon_sym_stakeholder);
      END_STATE();
    case 735:
      if (lookahead == 'e') ADVANCE(740);
      END_STATE();
    case 736:
      if (lookahead == 'n') ADVANCE(741);
      END_STATE();
    case 737:
      ACCEPT_TOKEN(anon_sym_multiplicity);
      END_STATE();
    case 738:
      ACCEPT_TOKEN(anon_sym_redefinition);
      END_STATE();
    case 739:
      if (lookahead == 'o') ADVANCE(742);
      END_STATE();
    case 740:
      if (lookahead == 'r') ADVANCE(743);
      END_STATE();
    case 741:
      ACCEPT_TOKEN(anon_sym_verification);
      END_STATE();
    case 742:
      if (lookahead == 'n') ADVANCE(744);
      END_STATE();
    case 743:
      ACCEPT_TOKEN(anon_sym_subclassifier);
      END_STATE();
    case 744:
      ACCEPT_TOKEN(anon_sym_specialization);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 12},
  [2] = {.lex_state = 12},
  [3] = {.lex_state = 12},
  [4] = {.lex_state = 12},
  [5] = {.lex_state = 12},
  [6] = {.lex_state = 12},
  [7] = {.lex_state = 12},
  [8] = {.lex_state = 12},
  [9] = {.lex_state = 12},
  [10] = {.lex_state = 13},
  [11] = {.lex_state = 13},
  [12] = {.lex_state = 13},
  [13] = {.lex_state = 12},
  [14] = {.lex_state = 12},
  [15] = {.lex_state = 12},
  [16] = {.lex_state = 12},
  [17] = {.lex_state = 12},
  [18] = {.lex_state = 12},
  [19] = {.lex_state = 13},
  [20] = {.lex_state = 12},
  [21] = {.lex_state = 12},
  [22] = {.lex_state = 12},
  [23] = {.lex_state = 12},
  [24] = {.lex_state = 12},
  [25] = {.lex_state = 12},
  [26] = {.lex_state = 12},
  [27] = {.lex_state = 12},
  [28] = {.lex_state = 12},
  [29] = {.lex_state = 12},
  [30] = {.lex_state = 12},
  [31] = {.lex_state = 12},
  [32] = {.lex_state = 12},
  [33] = {.lex_state = 12},
  [34] = {.lex_state = 12},
  [35] = {.lex_state = 12},
  [36] = {.lex_state = 12},
  [37] = {.lex_state = 12},
  [38] = {.lex_state = 12},
  [39] = {.lex_state = 12},
  [40] = {.lex_state = 12},
  [41] = {.lex_state = 12},
  [42] = {.lex_state = 12},
  [43] = {.lex_state = 12},
  [44] = {.lex_state = 12},
  [45] = {.lex_state = 12},
  [46] = {.lex_state = 12},
  [47] = {.lex_state = 12},
  [48] = {.lex_state = 12},
  [49] = {.lex_state = 12},
  [50] = {.lex_state = 12},
  [51] = {.lex_state = 12},
  [52] = {.lex_state = 12},
  [53] = {.lex_state = 12},
  [54] = {.lex_state = 12},
  [55] = {.lex_state = 12},
  [56] = {.lex_state = 12},
  [57] = {.lex_state = 5},
  [58] = {.lex_state = 12},
  [59] = {.lex_state = 12},
  [60] = {.lex_state = 12},
  [61] = {.lex_state = 12},
  [62] = {.lex_state = 12},
  [63] = {.lex_state = 12},
  [64] = {.lex_state = 12},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_package] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_part] = ACTIONS(1),
    [anon_sym_def] = ACTIONS(1),
    [anon_sym_attribute] = ACTIONS(1),
    [anon_sym_action] = ACTIONS(1),
    [anon_sym_state] = ACTIONS(1),
    [anon_sym_interface] = ACTIONS(1),
    [anon_sym_port] = ACTIONS(1),
    [anon_sym_requirement] = ACTIONS(1),
    [anon_sym_constraint] = ACTIONS(1),
    [anon_sym_enum] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [anon_sym_null] = ACTIONS(1),
    [anon_sym_about] = ACTIONS(1),
    [anon_sym_abstract] = ACTIONS(1),
    [anon_sym_accept] = ACTIONS(1),
    [anon_sym_actor] = ACTIONS(1),
    [anon_sym_after] = ACTIONS(1),
    [anon_sym_alias] = ACTIONS(1),
    [anon_sym_all] = ACTIONS(1),
    [anon_sym_allocate] = ACTIONS(1),
    [anon_sym_allocation] = ACTIONS(1),
    [anon_sym_analysis] = ACTIONS(1),
    [anon_sym_and] = ACTIONS(1),
    [anon_sym_as] = ACTIONS(1),
    [anon_sym_assert] = ACTIONS(1),
    [anon_sym_assign] = ACTIONS(1),
    [anon_sym_assoc] = ACTIONS(1),
    [anon_sym_assume] = ACTIONS(1),
    [anon_sym_at] = ACTIONS(1),
    [anon_sym_behavior] = ACTIONS(1),
    [anon_sym_bind] = ACTIONS(1),
    [anon_sym_binding] = ACTIONS(1),
    [anon_sym_bool] = ACTIONS(1),
    [anon_sym_by] = ACTIONS(1),
    [anon_sym_calc] = ACTIONS(1),
    [anon_sym_case] = ACTIONS(1),
    [anon_sym_chains] = ACTIONS(1),
    [anon_sym_class] = ACTIONS(1),
    [anon_sym_classifier] = ACTIONS(1),
    [anon_sym_comment] = ACTIONS(1),
    [anon_sym_composite] = ACTIONS(1),
    [anon_sym_concern] = ACTIONS(1),
    [anon_sym_conjugate] = ACTIONS(1),
    [anon_sym_conjugates] = ACTIONS(1),
    [anon_sym_conjugation] = ACTIONS(1),
    [anon_sym_connect] = ACTIONS(1),
    [anon_sym_connection] = ACTIONS(1),
    [anon_sym_connector] = ACTIONS(1),
    [anon_sym_const] = ACTIONS(1),
    [anon_sym_constant] = ACTIONS(1),
    [anon_sym_crosses] = ACTIONS(1),
    [anon_sym_datatype] = ACTIONS(1),
    [anon_sym_decide] = ACTIONS(1),
    [anon_sym_default] = ACTIONS(1),
    [anon_sym_defined] = ACTIONS(1),
    [anon_sym_dependency] = ACTIONS(1),
    [anon_sym_derived] = ACTIONS(1),
    [anon_sym_differences] = ACTIONS(1),
    [anon_sym_disjoining] = ACTIONS(1),
    [anon_sym_disjoint] = ACTIONS(1),
    [anon_sym_do] = ACTIONS(1),
    [anon_sym_doc] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_end] = ACTIONS(1),
    [anon_sym_entry] = ACTIONS(1),
    [anon_sym_event] = ACTIONS(1),
    [anon_sym_exhibit] = ACTIONS(1),
    [anon_sym_exit] = ACTIONS(1),
    [anon_sym_expose] = ACTIONS(1),
    [anon_sym_expr] = ACTIONS(1),
    [anon_sym_feature] = ACTIONS(1),
    [anon_sym_featured] = ACTIONS(1),
    [anon_sym_featuring] = ACTIONS(1),
    [anon_sym_filter] = ACTIONS(1),
    [anon_sym_first] = ACTIONS(1),
    [anon_sym_flow] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_fork] = ACTIONS(1),
    [anon_sym_frame] = ACTIONS(1),
    [anon_sym_from] = ACTIONS(1),
    [anon_sym_function] = ACTIONS(1),
    [anon_sym_hastype] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_implies] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_include] = ACTIONS(1),
    [anon_sym_individual] = ACTIONS(1),
    [anon_sym_inout] = ACTIONS(1),
    [anon_sym_interaction] = ACTIONS(1),
    [anon_sym_intersects] = ACTIONS(1),
    [anon_sym_inv] = ACTIONS(1),
    [anon_sym_inverse] = ACTIONS(1),
    [anon_sym_inverting] = ACTIONS(1),
    [anon_sym_istype] = ACTIONS(1),
    [anon_sym_item] = ACTIONS(1),
    [anon_sym_join] = ACTIONS(1),
    [anon_sym_language] = ACTIONS(1),
    [anon_sym_library] = ACTIONS(1),
    [anon_sym_locale] = ACTIONS(1),
    [anon_sym_loop] = ACTIONS(1),
    [anon_sym_member] = ACTIONS(1),
    [anon_sym_merge] = ACTIONS(1),
    [anon_sym_message] = ACTIONS(1),
    [anon_sym_meta] = ACTIONS(1),
    [anon_sym_metaclass] = ACTIONS(1),
    [anon_sym_metadata] = ACTIONS(1),
    [anon_sym_multiplicity] = ACTIONS(1),
    [anon_sym_namespace] = ACTIONS(1),
    [anon_sym_new] = ACTIONS(1),
    [anon_sym_nonunique] = ACTIONS(1),
    [anon_sym_not] = ACTIONS(1),
    [anon_sym_objective] = ACTIONS(1),
    [anon_sym_occurrence] = ACTIONS(1),
    [anon_sym_of] = ACTIONS(1),
    [anon_sym_or] = ACTIONS(1),
    [anon_sym_ordered] = ACTIONS(1),
    [anon_sym_out] = ACTIONS(1),
    [anon_sym_parallel] = ACTIONS(1),
    [anon_sym_perform] = ACTIONS(1),
    [anon_sym_portion] = ACTIONS(1),
    [anon_sym_predicate] = ACTIONS(1),
    [anon_sym_private] = ACTIONS(1),
    [anon_sym_protected] = ACTIONS(1),
    [anon_sym_public] = ACTIONS(1),
    [anon_sym_readonly] = ACTIONS(1),
    [anon_sym_redefines] = ACTIONS(1),
    [anon_sym_redefinition] = ACTIONS(1),
    [anon_sym_ref] = ACTIONS(1),
    [anon_sym_references] = ACTIONS(1),
    [anon_sym_render] = ACTIONS(1),
    [anon_sym_rendering] = ACTIONS(1),
    [anon_sym_rep] = ACTIONS(1),
    [anon_sym_require] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
    [anon_sym_satisfy] = ACTIONS(1),
    [anon_sym_send] = ACTIONS(1),
    [anon_sym_snapshot] = ACTIONS(1),
    [anon_sym_specialization] = ACTIONS(1),
    [anon_sym_specializes] = ACTIONS(1),
    [anon_sym_stakeholder] = ACTIONS(1),
    [anon_sym_standard] = ACTIONS(1),
    [anon_sym_step] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_subclassifier] = ACTIONS(1),
    [anon_sym_subject] = ACTIONS(1),
    [anon_sym_subset] = ACTIONS(1),
    [anon_sym_subsets] = ACTIONS(1),
    [anon_sym_subtype] = ACTIONS(1),
    [anon_sym_succession] = ACTIONS(1),
    [anon_sym_terminate] = ACTIONS(1),
    [anon_sym_then] = ACTIONS(1),
    [anon_sym_timeslice] = ACTIONS(1),
    [anon_sym_to] = ACTIONS(1),
    [anon_sym_transition] = ACTIONS(1),
    [anon_sym_typed] = ACTIONS(1),
    [anon_sym_typing] = ACTIONS(1),
    [anon_sym_unions] = ACTIONS(1),
    [anon_sym_until] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_var] = ACTIONS(1),
    [anon_sym_variant] = ACTIONS(1),
    [anon_sym_variation] = ACTIONS(1),
    [anon_sym_verification] = ACTIONS(1),
    [anon_sym_verify] = ACTIONS(1),
    [anon_sym_via] = ACTIONS(1),
    [anon_sym_view] = ACTIONS(1),
    [anon_sym_viewpoint] = ACTIONS(1),
    [anon_sym_when] = ACTIONS(1),
    [anon_sym_while] = ACTIONS(1),
    [anon_sym_xor] = ACTIONS(1),
    [anon_sym_EQ_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ_EQ] = ACTIONS(1),
    [anon_sym_QMARK_QMARK] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_AT_AT] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_STAR_STAR] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(62),
    [sym__statement] = STATE(4),
    [sym_package_decl] = STATE(4),
    [sym_import_decl] = STATE(4),
    [sym_part_def] = STATE(4),
    [sym_part_usage] = STATE(4),
    [sym_attribute_def] = STATE(4),
    [sym_attribute_usage] = STATE(4),
    [sym_definition] = STATE(4),
    [sym_usage] = STATE(4),
    [aux_sym_source_file_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_package] = ACTIONS(7),
    [anon_sym_import] = ACTIONS(9),
    [anon_sym_part] = ACTIONS(11),
    [anon_sym_attribute] = ACTIONS(13),
    [anon_sym_action] = ACTIONS(15),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_interface] = ACTIONS(15),
    [anon_sym_port] = ACTIONS(15),
    [anon_sym_requirement] = ACTIONS(15),
    [anon_sym_constraint] = ACTIONS(15),
    [anon_sym_enum] = ACTIONS(15),
    [anon_sym_type] = ACTIONS(15),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_package,
    ACTIONS(22), 1,
      anon_sym_import,
    ACTIONS(25), 1,
      anon_sym_part,
    ACTIONS(28), 1,
      anon_sym_attribute,
    ACTIONS(17), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    ACTIONS(31), 8,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
    STATE(2), 10,
      sym__statement,
      sym_package_decl,
      sym_import_decl,
      sym_part_def,
      sym_part_usage,
      sym_attribute_def,
      sym_attribute_usage,
      sym_definition,
      sym_usage,
      aux_sym_source_file_repeat1,
  [42] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_package,
    ACTIONS(9), 1,
      anon_sym_import,
    ACTIONS(11), 1,
      anon_sym_part,
    ACTIONS(13), 1,
      anon_sym_attribute,
    ACTIONS(34), 1,
      anon_sym_RBRACE,
    ACTIONS(15), 8,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
    STATE(5), 10,
      sym__statement,
      sym_package_decl,
      sym_import_decl,
      sym_part_def,
      sym_part_usage,
      sym_attribute_def,
      sym_attribute_usage,
      sym_definition,
      sym_usage,
      aux_sym_source_file_repeat1,
  [83] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_package,
    ACTIONS(9), 1,
      anon_sym_import,
    ACTIONS(11), 1,
      anon_sym_part,
    ACTIONS(13), 1,
      anon_sym_attribute,
    ACTIONS(36), 1,
      ts_builtin_sym_end,
    ACTIONS(15), 8,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
    STATE(2), 10,
      sym__statement,
      sym_package_decl,
      sym_import_decl,
      sym_part_def,
      sym_part_usage,
      sym_attribute_def,
      sym_attribute_usage,
      sym_definition,
      sym_usage,
      aux_sym_source_file_repeat1,
  [124] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_package,
    ACTIONS(9), 1,
      anon_sym_import,
    ACTIONS(11), 1,
      anon_sym_part,
    ACTIONS(13), 1,
      anon_sym_attribute,
    ACTIONS(38), 1,
      anon_sym_RBRACE,
    ACTIONS(15), 8,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
    STATE(2), 10,
      sym__statement,
      sym_package_decl,
      sym_import_decl,
      sym_part_def,
      sym_part_usage,
      sym_attribute_def,
      sym_attribute_usage,
      sym_definition,
      sym_usage,
      aux_sym_source_file_repeat1,
  [165] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(44), 1,
      anon_sym_SEMI,
    ACTIONS(46), 1,
      anon_sym_COLON,
    STATE(14), 1,
      sym_typing,
    STATE(33), 1,
      sym_block,
    ACTIONS(40), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [200] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(46), 1,
      anon_sym_COLON,
    ACTIONS(50), 1,
      anon_sym_SEMI,
    STATE(16), 1,
      sym_typing,
    STATE(26), 1,
      sym_block,
    ACTIONS(48), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [235] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(46), 1,
      anon_sym_COLON,
    ACTIONS(54), 1,
      anon_sym_SEMI,
    STATE(17), 1,
      sym_typing,
    STATE(28), 1,
      sym_block,
    ACTIONS(52), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [270] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(46), 1,
      anon_sym_COLON,
    ACTIONS(58), 1,
      anon_sym_SEMI,
    STATE(18), 1,
      sym_typing,
    STATE(25), 1,
      sym_block,
    ACTIONS(56), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [305] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(62), 1,
      anon_sym_COLON_COLON,
    STATE(12), 1,
      aux_sym_qualified_name_repeat1,
    ACTIONS(60), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [333] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(62), 1,
      anon_sym_COLON_COLON,
    STATE(10), 1,
      aux_sym_qualified_name_repeat1,
    ACTIONS(64), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [361] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(68), 1,
      anon_sym_COLON_COLON,
    STATE(12), 1,
      aux_sym_qualified_name_repeat1,
    ACTIONS(66), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [389] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(46), 1,
      anon_sym_COLON,
    ACTIONS(73), 1,
      anon_sym_SEMI,
    STATE(30), 1,
      sym_typing,
    ACTIONS(71), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [418] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(77), 1,
      anon_sym_SEMI,
    STATE(34), 1,
      sym_block,
    ACTIONS(75), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [447] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(46), 1,
      anon_sym_COLON,
    ACTIONS(81), 1,
      anon_sym_SEMI,
    STATE(24), 1,
      sym_typing,
    ACTIONS(79), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [476] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(85), 1,
      anon_sym_SEMI,
    STATE(29), 1,
      sym_block,
    ACTIONS(83), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [505] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(89), 1,
      anon_sym_SEMI,
    STATE(27), 1,
      sym_block,
    ACTIONS(87), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [534] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(93), 1,
      anon_sym_SEMI,
    STATE(31), 1,
      sym_block,
    ACTIONS(91), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [563] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(66), 17,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
      anon_sym_COLON_COLON,
  [586] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    STATE(36), 1,
      sym_block,
    ACTIONS(95), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [612] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(64), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [634] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(97), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [656] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(99), 15,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [677] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(103), 1,
      anon_sym_SEMI,
    ACTIONS(101), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [700] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(93), 1,
      anon_sym_SEMI,
    ACTIONS(91), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [723] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 1,
      anon_sym_SEMI,
    ACTIONS(83), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [746] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(107), 1,
      anon_sym_SEMI,
    ACTIONS(105), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [769] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_SEMI,
    ACTIONS(87), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [792] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(111), 1,
      anon_sym_SEMI,
    ACTIONS(109), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [815] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(115), 1,
      anon_sym_SEMI,
    ACTIONS(113), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [838] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(119), 1,
      anon_sym_SEMI,
    ACTIONS(117), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [861] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(121), 15,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_SEMI,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [882] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_SEMI,
    ACTIONS(75), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [905] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(125), 1,
      anon_sym_SEMI,
    ACTIONS(123), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [928] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(127), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [948] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(129), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [968] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [988] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(133), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1008] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(75), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1028] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(101), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1048] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(123), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1068] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(113), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1088] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(91), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1108] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(105), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1128] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(135), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1148] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(137), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1168] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1188] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(87), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1208] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(139), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1228] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1248] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1268] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(141), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_import,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [1288] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(143), 1,
      sym_identifier,
    STATE(21), 1,
      sym_qualified_name,
    STATE(22), 1,
      sym_type_ref,
  [1301] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(145), 1,
      sym_identifier,
    ACTIONS(147), 1,
      anon_sym_def,
  [1311] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 1,
      sym_identifier,
    ACTIONS(151), 1,
      anon_sym_def,
  [1321] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 1,
      sym_identifier,
    ACTIONS(155), 1,
      anon_sym_def,
  [1331] = 2,
    ACTIONS(157), 1,
      sym_import_path,
    ACTIONS(159), 1,
      sym_comment,
  [1338] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 1,
      anon_sym_SEMI,
  [1345] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(163), 1,
      sym_identifier,
  [1352] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      sym_identifier,
  [1359] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 1,
      sym_identifier,
  [1366] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 1,
      ts_builtin_sym_end,
  [1373] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 1,
      sym_identifier,
  [1380] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(173), 1,
      sym_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 42,
  [SMALL_STATE(4)] = 83,
  [SMALL_STATE(5)] = 124,
  [SMALL_STATE(6)] = 165,
  [SMALL_STATE(7)] = 200,
  [SMALL_STATE(8)] = 235,
  [SMALL_STATE(9)] = 270,
  [SMALL_STATE(10)] = 305,
  [SMALL_STATE(11)] = 333,
  [SMALL_STATE(12)] = 361,
  [SMALL_STATE(13)] = 389,
  [SMALL_STATE(14)] = 418,
  [SMALL_STATE(15)] = 447,
  [SMALL_STATE(16)] = 476,
  [SMALL_STATE(17)] = 505,
  [SMALL_STATE(18)] = 534,
  [SMALL_STATE(19)] = 563,
  [SMALL_STATE(20)] = 586,
  [SMALL_STATE(21)] = 612,
  [SMALL_STATE(22)] = 634,
  [SMALL_STATE(23)] = 656,
  [SMALL_STATE(24)] = 677,
  [SMALL_STATE(25)] = 700,
  [SMALL_STATE(26)] = 723,
  [SMALL_STATE(27)] = 746,
  [SMALL_STATE(28)] = 769,
  [SMALL_STATE(29)] = 792,
  [SMALL_STATE(30)] = 815,
  [SMALL_STATE(31)] = 838,
  [SMALL_STATE(32)] = 861,
  [SMALL_STATE(33)] = 882,
  [SMALL_STATE(34)] = 905,
  [SMALL_STATE(35)] = 928,
  [SMALL_STATE(36)] = 948,
  [SMALL_STATE(37)] = 968,
  [SMALL_STATE(38)] = 988,
  [SMALL_STATE(39)] = 1008,
  [SMALL_STATE(40)] = 1028,
  [SMALL_STATE(41)] = 1048,
  [SMALL_STATE(42)] = 1068,
  [SMALL_STATE(43)] = 1088,
  [SMALL_STATE(44)] = 1108,
  [SMALL_STATE(45)] = 1128,
  [SMALL_STATE(46)] = 1148,
  [SMALL_STATE(47)] = 1168,
  [SMALL_STATE(48)] = 1188,
  [SMALL_STATE(49)] = 1208,
  [SMALL_STATE(50)] = 1228,
  [SMALL_STATE(51)] = 1248,
  [SMALL_STATE(52)] = 1268,
  [SMALL_STATE(53)] = 1288,
  [SMALL_STATE(54)] = 1301,
  [SMALL_STATE(55)] = 1311,
  [SMALL_STATE(56)] = 1321,
  [SMALL_STATE(57)] = 1331,
  [SMALL_STATE(58)] = 1338,
  [SMALL_STATE(59)] = 1345,
  [SMALL_STATE(60)] = 1352,
  [SMALL_STATE(61)] = 1359,
  [SMALL_STATE(62)] = 1366,
  [SMALL_STATE(63)] = 1373,
  [SMALL_STATE(64)] = 1380,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(64),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(57),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(54),
  [28] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(55),
  [31] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(56),
  [34] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_usage, 2, 0, 1),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 3, 0, 3),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_usage, 2, 0, 1),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 3, 0, 3),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualified_name, 2, 0, 0),
  [62] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_ref, 1, 0, 0),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_qualified_name_repeat1, 2, 0, 0),
  [68] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_qualified_name_repeat1, 2, 0, 0), SHIFT_REPEAT(61),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_usage, 2, 0, 1),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_usage, 3, 0, 1),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_def, 3, 0, 3),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 4, 0, 3),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_usage, 3, 0, 1),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 4, 0, 3),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_decl, 2, 0, 1),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_typing, 2, 0, 4),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, 0, 0),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_def, 4, 0, 3),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_usage, 4, 0, 1),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 5, 0, 3),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_usage, 3, 0, 1),
  [115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 5, 0, 3),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2, 0, 0),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_usage, 4, 0, 1),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_usage, 5, 0, 1),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_decl, 3, 0, 1),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_decl, 3, 0, 2),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_usage, 4, 0, 1),
  [135] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 6, 0, 3),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_usage, 5, 0, 1),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_def, 5, 0, 3),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 6, 0, 3),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [145] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [147] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [149] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [151] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [153] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [155] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [157] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [159] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [169] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_sysml(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
