#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 54
#define LARGE_STATE_COUNT 11
#define SYMBOL_COUNT 37
#define ALIAS_COUNT 0
#define TOKEN_COUNT 24
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 2
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 4

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  anon_sym_package = 4,
  anon_sym_part = 5,
  anon_sym_def = 6,
  anon_sym_SEMI = 7,
  anon_sym_attribute = 8,
  anon_sym_import = 9,
  aux_sym_import_decl_token1 = 10,
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
  sym_comment = 23,
  sym_source_file = 24,
  sym__statement = 25,
  sym_block = 26,
  sym_package_decl = 27,
  sym_part_def = 28,
  sym_attribute_decl = 29,
  sym_import_decl = 30,
  sym_definition = 31,
  sym_typing = 32,
  sym_type_ref = 33,
  sym_qualified_name = 34,
  aux_sym_source_file_repeat1 = 35,
  aux_sym_qualified_name_repeat1 = 36,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_package] = "package",
  [anon_sym_part] = "part",
  [anon_sym_def] = "def",
  [anon_sym_SEMI] = ";",
  [anon_sym_attribute] = "attribute",
  [anon_sym_import] = "import",
  [aux_sym_import_decl_token1] = "import_decl_token1",
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
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym__statement] = "_statement",
  [sym_block] = "block",
  [sym_package_decl] = "package_decl",
  [sym_part_def] = "part_def",
  [sym_attribute_decl] = "attribute_decl",
  [sym_import_decl] = "import_decl",
  [sym_definition] = "definition",
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
  [anon_sym_part] = anon_sym_part,
  [anon_sym_def] = anon_sym_def,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_attribute] = anon_sym_attribute,
  [anon_sym_import] = anon_sym_import,
  [aux_sym_import_decl_token1] = aux_sym_import_decl_token1,
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
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym__statement] = sym__statement,
  [sym_block] = sym_block,
  [sym_package_decl] = sym_package_decl,
  [sym_part_def] = sym_part_def,
  [sym_attribute_decl] = sym_attribute_decl,
  [sym_import_decl] = sym_import_decl,
  [sym_definition] = sym_definition,
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
  [anon_sym_part] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_def] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_attribute] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_import_decl_token1] = {
    .visible = false,
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
  [sym_part_def] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_import_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_definition] = {
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
  field_type = 2,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_name] = "name",
  [field_type] = "type",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 1},
  [1] =
    {field_name, 2},
  [2] =
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(11);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(22);
      if (lookahead == ';') ADVANCE(14);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '}') ADVANCE(13);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(24);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(25);
      if (lookahead == '\\') ADVANCE(8);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(29);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(28);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '/') ADVANCE(16);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(20);
      END_STATE();
    case 6:
      if (lookahead == ':') ADVANCE(23);
      END_STATE();
    case 7:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      END_STATE();
    case 8:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(1);
      END_STATE();
    case 9:
      if (eof) ADVANCE(11);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(21);
      if (lookahead == ';') ADVANCE(14);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '}') ADVANCE(13);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(9);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(24);
      END_STATE();
    case 10:
      if (eof) ADVANCE(11);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(6);
      if (lookahead == ';') ADVANCE(14);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '}') ADVANCE(13);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(10);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(24);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(aux_sym_import_decl_token1);
      if (lookahead == '\n') ADVANCE(20);
      if (lookahead == ';') ADVANCE(29);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(aux_sym_import_decl_token1);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(15);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(20);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(aux_sym_import_decl_token1);
      if (lookahead == '*') ADVANCE(17);
      if (lookahead == '/') ADVANCE(20);
      if (lookahead == ';') ADVANCE(4);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(aux_sym_import_decl_token1);
      if (lookahead == '*') ADVANCE(17);
      if (lookahead == ';') ADVANCE(4);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(aux_sym_import_decl_token1);
      if (lookahead == '/') ADVANCE(16);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(aux_sym_import_decl_token1);
      if (lookahead != 0 &&
          lookahead != ';') ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(23);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(24);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(7);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(29);
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
        'c', 2,
        'd', 3,
        'e', 4,
        'i', 5,
        'p', 6,
        'r', 7,
        's', 8,
        't', 9,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'c') ADVANCE(10);
      if (lookahead == 't') ADVANCE(11);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(12);
      END_STATE();
    case 3:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 4:
      if (lookahead == 'n') ADVANCE(14);
      END_STATE();
    case 5:
      if (lookahead == 'm') ADVANCE(15);
      if (lookahead == 'n') ADVANCE(16);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(17);
      if (lookahead == 'o') ADVANCE(18);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(19);
      END_STATE();
    case 8:
      if (lookahead == 't') ADVANCE(20);
      END_STATE();
    case 9:
      if (lookahead == 'y') ADVANCE(21);
      END_STATE();
    case 10:
      if (lookahead == 't') ADVANCE(22);
      END_STATE();
    case 11:
      if (lookahead == 't') ADVANCE(23);
      END_STATE();
    case 12:
      if (lookahead == 'n') ADVANCE(24);
      END_STATE();
    case 13:
      if (lookahead == 'f') ADVANCE(25);
      END_STATE();
    case 14:
      if (lookahead == 'u') ADVANCE(26);
      END_STATE();
    case 15:
      if (lookahead == 'p') ADVANCE(27);
      END_STATE();
    case 16:
      if (lookahead == 't') ADVANCE(28);
      END_STATE();
    case 17:
      if (lookahead == 'c') ADVANCE(29);
      if (lookahead == 'r') ADVANCE(30);
      END_STATE();
    case 18:
      if (lookahead == 'r') ADVANCE(31);
      END_STATE();
    case 19:
      if (lookahead == 'q') ADVANCE(32);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(33);
      END_STATE();
    case 21:
      if (lookahead == 'p') ADVANCE(34);
      END_STATE();
    case 22:
      if (lookahead == 'i') ADVANCE(35);
      END_STATE();
    case 23:
      if (lookahead == 'r') ADVANCE(36);
      END_STATE();
    case 24:
      if (lookahead == 's') ADVANCE(37);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_def);
      END_STATE();
    case 26:
      if (lookahead == 'm') ADVANCE(38);
      END_STATE();
    case 27:
      if (lookahead == 'o') ADVANCE(39);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(40);
      END_STATE();
    case 29:
      if (lookahead == 'k') ADVANCE(41);
      END_STATE();
    case 30:
      if (lookahead == 't') ADVANCE(42);
      END_STATE();
    case 31:
      if (lookahead == 't') ADVANCE(43);
      END_STATE();
    case 32:
      if (lookahead == 'u') ADVANCE(44);
      END_STATE();
    case 33:
      if (lookahead == 't') ADVANCE(45);
      END_STATE();
    case 34:
      if (lookahead == 'e') ADVANCE(46);
      END_STATE();
    case 35:
      if (lookahead == 'o') ADVANCE(47);
      END_STATE();
    case 36:
      if (lookahead == 'i') ADVANCE(48);
      END_STATE();
    case 37:
      if (lookahead == 't') ADVANCE(49);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_enum);
      END_STATE();
    case 39:
      if (lookahead == 'r') ADVANCE(50);
      END_STATE();
    case 40:
      if (lookahead == 'r') ADVANCE(51);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(52);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_part);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_port);
      END_STATE();
    case 44:
      if (lookahead == 'i') ADVANCE(53);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(54);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    case 47:
      if (lookahead == 'n') ADVANCE(55);
      END_STATE();
    case 48:
      if (lookahead == 'b') ADVANCE(56);
      END_STATE();
    case 49:
      if (lookahead == 'r') ADVANCE(57);
      END_STATE();
    case 50:
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 51:
      if (lookahead == 'f') ADVANCE(59);
      END_STATE();
    case 52:
      if (lookahead == 'g') ADVANCE(60);
      END_STATE();
    case 53:
      if (lookahead == 'r') ADVANCE(61);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_action);
      END_STATE();
    case 56:
      if (lookahead == 'u') ADVANCE(62);
      END_STATE();
    case 57:
      if (lookahead == 'a') ADVANCE(63);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 59:
      if (lookahead == 'a') ADVANCE(64);
      END_STATE();
    case 60:
      if (lookahead == 'e') ADVANCE(65);
      END_STATE();
    case 61:
      if (lookahead == 'e') ADVANCE(66);
      END_STATE();
    case 62:
      if (lookahead == 't') ADVANCE(67);
      END_STATE();
    case 63:
      if (lookahead == 'i') ADVANCE(68);
      END_STATE();
    case 64:
      if (lookahead == 'c') ADVANCE(69);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_package);
      END_STATE();
    case 66:
      if (lookahead == 'm') ADVANCE(70);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(71);
      END_STATE();
    case 68:
      if (lookahead == 'n') ADVANCE(72);
      END_STATE();
    case 69:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(74);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 72:
      if (lookahead == 't') ADVANCE(75);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 74:
      if (lookahead == 'n') ADVANCE(76);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_constraint);
      END_STATE();
    case 76:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_requirement);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 9},
  [7] = {.lex_state = 9},
  [8] = {.lex_state = 10},
  [9] = {.lex_state = 10},
  [10] = {.lex_state = 10},
  [11] = {.lex_state = 9},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 10},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 5},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_package] = ACTIONS(1),
    [anon_sym_part] = ACTIONS(1),
    [anon_sym_def] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_attribute] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
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
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(46),
    [sym__statement] = STATE(3),
    [sym_package_decl] = STATE(3),
    [sym_part_def] = STATE(3),
    [sym_attribute_decl] = STATE(3),
    [sym_import_decl] = STATE(3),
    [sym_definition] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_package] = ACTIONS(7),
    [anon_sym_part] = ACTIONS(9),
    [anon_sym_attribute] = ACTIONS(11),
    [anon_sym_import] = ACTIONS(13),
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
  [2] = {
    [sym__statement] = STATE(2),
    [sym_package_decl] = STATE(2),
    [sym_part_def] = STATE(2),
    [sym_attribute_decl] = STATE(2),
    [sym_import_decl] = STATE(2),
    [sym_definition] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(17),
    [anon_sym_RBRACE] = ACTIONS(17),
    [anon_sym_package] = ACTIONS(19),
    [anon_sym_part] = ACTIONS(22),
    [anon_sym_attribute] = ACTIONS(25),
    [anon_sym_import] = ACTIONS(28),
    [anon_sym_action] = ACTIONS(31),
    [anon_sym_state] = ACTIONS(31),
    [anon_sym_interface] = ACTIONS(31),
    [anon_sym_port] = ACTIONS(31),
    [anon_sym_requirement] = ACTIONS(31),
    [anon_sym_constraint] = ACTIONS(31),
    [anon_sym_enum] = ACTIONS(31),
    [anon_sym_type] = ACTIONS(31),
    [sym_comment] = ACTIONS(3),
  },
  [3] = {
    [sym__statement] = STATE(2),
    [sym_package_decl] = STATE(2),
    [sym_part_def] = STATE(2),
    [sym_attribute_decl] = STATE(2),
    [sym_import_decl] = STATE(2),
    [sym_definition] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(34),
    [anon_sym_package] = ACTIONS(7),
    [anon_sym_part] = ACTIONS(9),
    [anon_sym_attribute] = ACTIONS(11),
    [anon_sym_import] = ACTIONS(13),
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
  [4] = {
    [sym__statement] = STATE(5),
    [sym_package_decl] = STATE(5),
    [sym_part_def] = STATE(5),
    [sym_attribute_decl] = STATE(5),
    [sym_import_decl] = STATE(5),
    [sym_definition] = STATE(5),
    [aux_sym_source_file_repeat1] = STATE(5),
    [anon_sym_RBRACE] = ACTIONS(36),
    [anon_sym_package] = ACTIONS(7),
    [anon_sym_part] = ACTIONS(9),
    [anon_sym_attribute] = ACTIONS(11),
    [anon_sym_import] = ACTIONS(13),
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
  [5] = {
    [sym__statement] = STATE(2),
    [sym_package_decl] = STATE(2),
    [sym_part_def] = STATE(2),
    [sym_attribute_decl] = STATE(2),
    [sym_import_decl] = STATE(2),
    [sym_definition] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [anon_sym_RBRACE] = ACTIONS(38),
    [anon_sym_package] = ACTIONS(7),
    [anon_sym_part] = ACTIONS(9),
    [anon_sym_attribute] = ACTIONS(11),
    [anon_sym_import] = ACTIONS(13),
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
  [6] = {
    [sym_block] = STATE(22),
    [sym_typing] = STATE(14),
    [ts_builtin_sym_end] = ACTIONS(40),
    [anon_sym_LBRACE] = ACTIONS(42),
    [anon_sym_RBRACE] = ACTIONS(40),
    [anon_sym_package] = ACTIONS(40),
    [anon_sym_part] = ACTIONS(40),
    [anon_sym_SEMI] = ACTIONS(44),
    [anon_sym_attribute] = ACTIONS(40),
    [anon_sym_import] = ACTIONS(40),
    [anon_sym_action] = ACTIONS(40),
    [anon_sym_state] = ACTIONS(40),
    [anon_sym_interface] = ACTIONS(40),
    [anon_sym_port] = ACTIONS(40),
    [anon_sym_requirement] = ACTIONS(40),
    [anon_sym_constraint] = ACTIONS(40),
    [anon_sym_enum] = ACTIONS(40),
    [anon_sym_type] = ACTIONS(40),
    [anon_sym_COLON] = ACTIONS(46),
    [sym_comment] = ACTIONS(3),
  },
  [7] = {
    [sym_block] = STATE(21),
    [sym_typing] = STATE(13),
    [ts_builtin_sym_end] = ACTIONS(48),
    [anon_sym_LBRACE] = ACTIONS(42),
    [anon_sym_RBRACE] = ACTIONS(48),
    [anon_sym_package] = ACTIONS(48),
    [anon_sym_part] = ACTIONS(48),
    [anon_sym_SEMI] = ACTIONS(50),
    [anon_sym_attribute] = ACTIONS(48),
    [anon_sym_import] = ACTIONS(48),
    [anon_sym_action] = ACTIONS(48),
    [anon_sym_state] = ACTIONS(48),
    [anon_sym_interface] = ACTIONS(48),
    [anon_sym_port] = ACTIONS(48),
    [anon_sym_requirement] = ACTIONS(48),
    [anon_sym_constraint] = ACTIONS(48),
    [anon_sym_enum] = ACTIONS(48),
    [anon_sym_type] = ACTIONS(48),
    [anon_sym_COLON] = ACTIONS(46),
    [sym_comment] = ACTIONS(3),
  },
  [8] = {
    [aux_sym_qualified_name_repeat1] = STATE(9),
    [ts_builtin_sym_end] = ACTIONS(52),
    [anon_sym_LBRACE] = ACTIONS(52),
    [anon_sym_RBRACE] = ACTIONS(52),
    [anon_sym_package] = ACTIONS(52),
    [anon_sym_part] = ACTIONS(52),
    [anon_sym_SEMI] = ACTIONS(52),
    [anon_sym_attribute] = ACTIONS(52),
    [anon_sym_import] = ACTIONS(52),
    [anon_sym_action] = ACTIONS(52),
    [anon_sym_state] = ACTIONS(52),
    [anon_sym_interface] = ACTIONS(52),
    [anon_sym_port] = ACTIONS(52),
    [anon_sym_requirement] = ACTIONS(52),
    [anon_sym_constraint] = ACTIONS(52),
    [anon_sym_enum] = ACTIONS(52),
    [anon_sym_type] = ACTIONS(52),
    [anon_sym_COLON_COLON] = ACTIONS(54),
    [sym_comment] = ACTIONS(3),
  },
  [9] = {
    [aux_sym_qualified_name_repeat1] = STATE(10),
    [ts_builtin_sym_end] = ACTIONS(56),
    [anon_sym_LBRACE] = ACTIONS(56),
    [anon_sym_RBRACE] = ACTIONS(56),
    [anon_sym_package] = ACTIONS(56),
    [anon_sym_part] = ACTIONS(56),
    [anon_sym_SEMI] = ACTIONS(56),
    [anon_sym_attribute] = ACTIONS(56),
    [anon_sym_import] = ACTIONS(56),
    [anon_sym_action] = ACTIONS(56),
    [anon_sym_state] = ACTIONS(56),
    [anon_sym_interface] = ACTIONS(56),
    [anon_sym_port] = ACTIONS(56),
    [anon_sym_requirement] = ACTIONS(56),
    [anon_sym_constraint] = ACTIONS(56),
    [anon_sym_enum] = ACTIONS(56),
    [anon_sym_type] = ACTIONS(56),
    [anon_sym_COLON_COLON] = ACTIONS(54),
    [sym_comment] = ACTIONS(3),
  },
  [10] = {
    [aux_sym_qualified_name_repeat1] = STATE(10),
    [ts_builtin_sym_end] = ACTIONS(58),
    [anon_sym_LBRACE] = ACTIONS(58),
    [anon_sym_RBRACE] = ACTIONS(58),
    [anon_sym_package] = ACTIONS(58),
    [anon_sym_part] = ACTIONS(58),
    [anon_sym_SEMI] = ACTIONS(58),
    [anon_sym_attribute] = ACTIONS(58),
    [anon_sym_import] = ACTIONS(58),
    [anon_sym_action] = ACTIONS(58),
    [anon_sym_state] = ACTIONS(58),
    [anon_sym_interface] = ACTIONS(58),
    [anon_sym_port] = ACTIONS(58),
    [anon_sym_requirement] = ACTIONS(58),
    [anon_sym_constraint] = ACTIONS(58),
    [anon_sym_enum] = ACTIONS(58),
    [anon_sym_type] = ACTIONS(58),
    [anon_sym_COLON_COLON] = ACTIONS(60),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(46), 1,
      anon_sym_COLON,
    ACTIONS(65), 1,
      anon_sym_SEMI,
    STATE(23), 1,
      sym_typing,
    ACTIONS(63), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [29] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(69), 1,
      anon_sym_SEMI,
    STATE(25), 1,
      sym_block,
    ACTIONS(67), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [58] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(73), 1,
      anon_sym_SEMI,
    STATE(26), 1,
      sym_block,
    ACTIONS(71), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [87] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(77), 1,
      anon_sym_SEMI,
    STATE(28), 1,
      sym_block,
    ACTIONS(75), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [116] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    ACTIONS(81), 1,
      anon_sym_SEMI,
    STATE(20), 1,
      sym_block,
    ACTIONS(79), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [145] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(58), 17,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_SEMI,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
      anon_sym_COLON_COLON,
  [168] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(42), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      sym_block,
    ACTIONS(83), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [194] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(52), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_SEMI,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [216] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 16,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_SEMI,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [238] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_SEMI,
    ACTIONS(87), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [261] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(73), 1,
      anon_sym_SEMI,
    ACTIONS(71), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [284] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_SEMI,
    ACTIONS(75), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [307] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(93), 1,
      anon_sym_SEMI,
    ACTIONS(91), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [330] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 15,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_SEMI,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [351] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(99), 1,
      anon_sym_SEMI,
    ACTIONS(97), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [374] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(103), 1,
      anon_sym_SEMI,
    ACTIONS(101), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [397] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(105), 15,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_SEMI,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [418] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 1,
      anon_sym_SEMI,
    ACTIONS(107), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [441] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(91), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [461] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(111), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [481] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(75), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [501] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(97), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [521] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(101), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [541] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(87), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [561] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(113), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [581] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(107), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [601] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(115), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [621] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [641] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(119), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [661] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(121), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [681] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(123), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [701] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(71), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_package,
      anon_sym_part,
      anon_sym_attribute,
      anon_sym_import,
      anon_sym_action,
      anon_sym_state,
      anon_sym_interface,
      anon_sym_port,
      anon_sym_requirement,
      anon_sym_constraint,
      anon_sym_enum,
      anon_sym_type,
  [721] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(125), 1,
      sym_identifier,
    STATE(18), 1,
      sym_qualified_name,
    STATE(19), 1,
      sym_type_ref,
  [734] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(127), 1,
      sym_identifier,
    ACTIONS(129), 1,
      anon_sym_def,
  [744] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 1,
      sym_identifier,
    ACTIONS(133), 1,
      anon_sym_def,
  [754] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(135), 1,
      ts_builtin_sym_end,
  [761] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(137), 1,
      sym_identifier,
  [768] = 2,
    ACTIONS(139), 1,
      aux_sym_import_decl_token1,
    ACTIONS(141), 1,
      sym_comment,
  [775] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(143), 1,
      sym_identifier,
  [782] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(145), 1,
      anon_sym_SEMI,
  [789] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(147), 1,
      sym_identifier,
  [796] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 1,
      sym_identifier,
  [803] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(151), 1,
      sym_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(11)] = 0,
  [SMALL_STATE(12)] = 29,
  [SMALL_STATE(13)] = 58,
  [SMALL_STATE(14)] = 87,
  [SMALL_STATE(15)] = 116,
  [SMALL_STATE(16)] = 145,
  [SMALL_STATE(17)] = 168,
  [SMALL_STATE(18)] = 194,
  [SMALL_STATE(19)] = 216,
  [SMALL_STATE(20)] = 238,
  [SMALL_STATE(21)] = 261,
  [SMALL_STATE(22)] = 284,
  [SMALL_STATE(23)] = 307,
  [SMALL_STATE(24)] = 330,
  [SMALL_STATE(25)] = 351,
  [SMALL_STATE(26)] = 374,
  [SMALL_STATE(27)] = 397,
  [SMALL_STATE(28)] = 418,
  [SMALL_STATE(29)] = 441,
  [SMALL_STATE(30)] = 461,
  [SMALL_STATE(31)] = 481,
  [SMALL_STATE(32)] = 501,
  [SMALL_STATE(33)] = 521,
  [SMALL_STATE(34)] = 541,
  [SMALL_STATE(35)] = 561,
  [SMALL_STATE(36)] = 581,
  [SMALL_STATE(37)] = 601,
  [SMALL_STATE(38)] = 621,
  [SMALL_STATE(39)] = 641,
  [SMALL_STATE(40)] = 661,
  [SMALL_STATE(41)] = 681,
  [SMALL_STATE(42)] = 701,
  [SMALL_STATE(43)] = 721,
  [SMALL_STATE(44)] = 734,
  [SMALL_STATE(45)] = 744,
  [SMALL_STATE(46)] = 754,
  [SMALL_STATE(47)] = 761,
  [SMALL_STATE(48)] = 768,
  [SMALL_STATE(49)] = 775,
  [SMALL_STATE(50)] = 782,
  [SMALL_STATE(51)] = 789,
  [SMALL_STATE(52)] = 796,
  [SMALL_STATE(53)] = 803,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(51),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(45),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(52),
  [28] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(48),
  [31] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(44),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 3, 0, 2),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 2, 0, 1),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_ref, 1, 0, 0),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualified_name, 2, 0, 0),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_qualified_name_repeat1, 2, 0, 0),
  [60] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_qualified_name_repeat1, 2, 0, 0), SHIFT_REPEAT(47),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_decl, 2, 0, 1),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 2, 0, 1),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 3, 0, 1),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 4, 0, 2),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 3, 0, 2),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_decl, 2, 0, 1),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_typing, 2, 0, 3),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 4, 0, 2),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_decl, 3, 0, 1),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2, 0, 0),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 3, 0, 1),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 4, 0, 1),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, 0, 0),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 5, 0, 2),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_decl, 3, 0, 0),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_decl, 4, 0, 1),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 4, 0, 1),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_decl, 3, 0, 1),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 5, 0, 1),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 5, 0, 2),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_part_def, 6, 0, 2),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [127] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [129] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [131] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [133] = {.entry = {.count = 1, .reusable = false}}, SHIFT(49),
  [135] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [139] = {.entry = {.count = 1, .reusable = false}}, SHIFT(50),
  [141] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [147] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [151] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
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
