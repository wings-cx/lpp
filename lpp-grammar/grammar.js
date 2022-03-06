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

    _statement: $ =>
      choice(
        $.local_assign_statement,
        $.assign_statement,
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

    expression_list: $ => 
      delim_rule($._expression, ","),
    
    _expression: $ => 
      choice(
        $._variable,
        $.number,
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
