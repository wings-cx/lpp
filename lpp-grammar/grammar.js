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

    number: $ => /-?([0-9]+(.[0-9]+)?)|(.[0-9]+)/,
  }
});
