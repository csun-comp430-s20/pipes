pub mod tokenizer {
    pub enum token {
        if_tk,              // if
        elif_tk,            // elif
        else_tk,            // else

        for_tk,             // for
        in_tk,              // in
        while_tk,           // while

        return_tk,          // return
        output_tk,          // ->
        let_tk,             // let
        assign_tk,          // =

        struct_tk,          // struct
        int_tk(i32),        // int
        bool_tk(bool),      // bool
        str_tk(String),     // str

        left_curly_tk,      // {
        left_brace_tk,      // [
        left_paren_tk,      // (
        right_curly_tk,     // }
        right_brace_tk,     // ]
        right_paren_tk,     // )

        dot_tk,             // .
        comma_tk,           // ,
        colon_tk,           // :
        semicolon_tk,       // ;

        minus_tk,           // -
        plus_tk,            // +
        divide_tk,          // /
        multiply_tk,        // *
        modulo_tk,          // %

        and_tk,             // &&
        or_tk,              // ||
        not_tk,             // !

        greater_than_tk,    // >
        less_than_tk,       // <
        greater_equal_tk,   // >=
        less_equal_tk,      // <=
        equal_tk,           // ==
        not_equal_tk,       // !=
    }
}
