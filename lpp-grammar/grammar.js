const delim_rule = (rule, delim) => 
  seq(rule, repeat(seq(delim, rule)));

module.exports = grammar({
  name: "lpp",

  rules: {
    chunk: $ =>
      seq(
        repeat(alias($.statement, $._statement)), 
        optional($.return_statement)
      ),

    statement: $ =>
      choice(
        $.local_assign_statement,
      ),

    return_statement: $ => 
      "return",

    local_assign_statement: $ =>
      seq(
        "local", 
        $.identifier_list,
        optional(seq(
          "=",
          $.expression_list,
        ))
      ),

    expression_list: $ => delim_rule($.expression, ","),
    
    expression: $ => 
      choice(
        $._variable,
        $.number,
      ),

    _variable: $ =>
      choice(
        $.variable_dot_index,
        $.identifier,
      ),

    variable_dot_index: $ =>
      seq(field("left", $._variable), ".", field("right", $.identifier)),

    identifier_list: $ => delim_rule($.identifier, ","),

    identifier: $ => /[a-zA-Z]+/,

    number: $ => /-?([0-9]+(.[0-9]+)?)|(.[0-9]+)/,
  }
});
