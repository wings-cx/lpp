const PRECEDENCE = {
  OR: 10,
  AND: 11,
  COMPARE: 12,
  BIT_OR: 13,
  BIT_NOT: 14,
  BIT_AND: 15,
  SHIFT: 16,
  CONCAT: 17,
  PLUS: 18,
  MULTI: 19,
  UNARY: 20,
  POWER: 21,
};

const delim_rule = (rule, delim) => 
  seq(rule, repeat(seq(delim, rule)));

module.exports = grammar({
  name: "lpp",

  rules: {
    chunk: $ =>
      seq(
        repeat($._statement),
        optional($.return_statement)
      ),

    terminated_chunk: $ =>
      seq(
        repeat($._statement),
        optional($.return_statement),
        "end"
      ),

    _statement: $ =>
      choice(
        $.local_assign_statement,
        $.assign_statement,
        $.function_declaration_statement,
      ),

    return_statement: $ => 
      "return",

    local_assign_statement: $ =>
      seq(
        "local", 
        field("identifiers", $.identifier_list),
        optional(seq(
          "=",
          field("expressions", $.expression_list),
        )),
      ),

    assign_statement: $ =>
      seq(
        field("variables", $.variable_list),
        "=",
        field("expressions", $.expression_list),
      ),

    function_declaration_statement: $ => 
      seq(
        "function",
        field("name", $._function_name),
        "()",
        field("body", $.terminated_chunk),
      ),

    _function_name: $ =>
      choice(
        $.function_name_dot_index,
        $.identifier,
      ),

    function_name_dot_index: $ =>
      seq(
        field("variable", $._function_name),
        ".", 
        field("identifier", $.identifier), 
        optional($.function_name_self_index),
      ),

    function_name_self_index: $ =>
      seq(
        ":",
        field("identifier", $.identifier)
      ),

    expression_list: $ => 
      delim_rule($._expression, ","),
    
    _expression: $ => 
      choice(
        $._variable,
        $.binary_expression,
        $.number,
      ),

    binary_expression: $ =>
      choice(
        ...[
          ["or", PRECEDENCE.OR],
          ["and", PRECEDENCE.AND],
          ["<", PRECEDENCE.COMPARE],
          ["<=", PRECEDENCE.COMPARE],
          ["==", PRECEDENCE.COMPARE],
          ["~=", PRECEDENCE.COMPARE],
          [">=", PRECEDENCE.COMPARE],
          [">", PRECEDENCE.COMPARE],
          ["+", PRECEDENCE.PLUS],
          ["-", PRECEDENCE.PLUS],
          ["*", PRECEDENCE.MULTI],
          ["/", PRECEDENCE.MULTI],
        ].map(([sym, precedence]) => 
          prec.left(precedence, seq($._expression, sym, $._expression))
        ),

        ...[
          ["..", PRECEDENCE.CONCAT],
          ["^", PRECEDENCE.POWER],
        ].map(([sym, precedence]) => 
          prec.right(precedence, seq($._expression, sym, $._expression)),
        ),
      ),

    variable_list: $ => 
      delim_rule($._variable, ","),

    _variable: $ =>
      choice(
        $.variable_dot_index,
        $.variable_sub_index,
        $.identifier,
      ),

    variable_dot_index: $ =>
      seq(field("variable", $._variable), ".", field("identifier", $.identifier)),

    variable_sub_index: $ =>
      seq(field("variable", $._variable), "[", field("expression", $._expression), "]"),

    identifier_list: $ => 
      delim_rule($.identifier, ","),

    identifier: $ => /[a-zA-Z]+/,

    number: $ => {
      const decimal_digits = /[0-9]+/
      const signed_integer = seq(optional(choice('-', '+')), decimal_digits)
      const decimal_exponent_part = seq(choice('e', 'E'), signed_integer)

      const decimal_integer_literal = choice(
        '0',
        seq(optional('0'), /[1-9]/, optional(decimal_digits))
      )

      const hex_digits = /[a-fA-F0-9]+/
      const hex_exponent_part = seq(choice('p', 'P'), signed_integer)

      const decimal_literal = choice(
        seq(decimal_integer_literal, '.', optional(decimal_digits), optional(decimal_exponent_part)),
        seq('.', decimal_digits, optional(decimal_exponent_part)),
        seq(decimal_integer_literal, optional(decimal_exponent_part))
      )

      const hex_literal = seq(
        choice('0x', '0X'),
        hex_digits,
        optional(seq('.', hex_digits)),
        optional(hex_exponent_part)
      )

      return token(choice(
        decimal_literal,
        hex_literal
      ))
    },
  }
});
