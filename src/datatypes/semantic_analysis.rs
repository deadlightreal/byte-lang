use super::{Statements, DeclareVariableType, Expression, Literal, Token, Statement};

pub struct SemanticAnaytis<'a> {
    input: &'a Vec<Statement>,
    untokenized_input: &'a str,
    position: usize
}

impl<'a> SemanticAnaytis<'a> {
    pub fn new(input: &'a Vec<Statement>, untokenized_input: &'a str) -> Self {
        return Self{input, untokenized_input, position: 0};
    }

    pub fn analyze_all(&mut self) {
        loop {
            match self.analyze_next() {
                Ok(r#continue) => {
                    if r#continue == false {
                        break;
                    }
                },
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            };
        };
    }

    pub fn analyze_next(&mut self) -> Result<bool, String> {
        let statement = self.input.get(self.position).unwrap();

        match statement.statement_type.clone() {
            Statements::VariableDeclaration(var) => {
                if((var.variable_type == DeclareVariableType::String && matches!(var.value, Expression::Literal(Literal::String(_))))
                    ||
                    (var.variable_type == DeclareVariableType::Number && matches!(var.value, Expression::Literal(Literal::Number(_))))) == false {
                    return Err(String::from(format!("Invalid Variable Declaration at line {} and col {}: {:?}", statement.line, statement.col, self.untokenized_input.get(statement.start_pos..statement.end_pos).unwrap()))); 
                }
            },
            Statements::EOF => {
                return Ok(false);
            },
            _ => ()
        };

        self.position += 1;

        return Ok(true);
    }
}
