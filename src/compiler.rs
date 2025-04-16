use std::collections::HashMap;

use crate::ast::{Statement, Expression};
use std::slice::Iter;
use std::{self, iter::Peekable};

pub struct Compiler {
    offset_map: HashMap<String, i32>,
    assem: Vec<String>,
    rodata: Vec<String>,
    string_constants: HashMap<String, String>,
    string_count: i32,
    var_offset: i32,
    label_count: i32,
}


impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            offset_map: HashMap::new(),
            assem: Vec::new(),
            rodata: Vec::new(),
            string_constants: HashMap::new(),
            string_count: 0,
            var_offset: 8,
            label_count: 0,
        }
    }
    pub fn compile(&mut self, ast: Vec<Statement>) -> Vec<String> {
        let mut iter = ast.iter().peekable();
        self.compiler(&mut iter);
        
        let mut result = Vec::new();
        result.extend(self.emit_data().clone());
        result.extend(self.assem.clone());

       result 
    }

    fn new_label(&mut self, label: &str) -> String {
        let label = format!("{}_{}", label, self.label_count);
        self.label_count += 1;
        label
    }

    fn register_string_literal(&mut self, s: &str) -> String {
        if let Some(label) = self.string_constants.get(s) {
            return label.clone();
        }

        let label = format!("str_{}", self.string_count);
        self.string_count += 1;
        self.string_constants.insert(s.to_string(), label.clone());

        self.rodata.push(format!("{}: db \"{}\", 0", label, s.replace("\"", "\\\"")));
        label
    }
    

    fn compiler(&mut self, iter: &mut Peekable<Iter<Statement>>) {
        while let Some(stmt) = iter.peek() {
            match stmt {
                Statement::Function { .. } => {
                    self.compile_function(iter);
                }
                _ => {
                }
            }
            iter.next();
        }
    }


    fn emit_data(&mut self) -> Vec<String> {
        let mut data = Vec::new();
        // Allows the use of printf if gcc is used to link
        data.push("extern _printf".into());
        data.push("section .rodata".into());
        // String required for printf to print ints
        data.push("fmt: db \"%ld\", 10, 0".to_string());

        // String required for printf to print string 
        data.push("fmt_str: db \"%s\", 10, 0".to_string());

        // Inject all string literals here
        data.extend(self.rodata.clone());

        // main and text section
        data.push("global _main".into());
        // Main section of the code
        data.push("section .text".into());

        data
    }

    fn compile_function(&mut self, iter: &mut Peekable<Iter<Statement>>) {
        if let Some(Statement::Function { name, params, body }) = iter.peek() {
            // Function has to be called main for program to run
            if name != "main" {
                self.assem.push(format!("global _{}", name));
            }
            // Prologue
            self.assem.push(format!("_{}:", name));
            // Pushing the previous call frame on to the stack - saving the previous base pointer
            self.assem.push("    push rbp".into());
            // Sets rbp to the current stack pointer - setting up the new call frame
            self.assem.push("    mov rbp, rsp".into());
            // reserve space 
            // reserving space for local variables
            self.assem.push("    sub rsp, 24".into());
            self.compile_statement(body);
            // Epilogue - cleaning up the stack 
            self.assem.push("    mov rsp, rbp".into());
            // TODO:  temp returning value
            self.assem.push("    mov rax, 0".into());
            self.assem.push("    pop rbp".into());
            self.assem.push("    ret".into());

        } else {
            self.assem.push("Error: No function found\n".to_string());
        }

    }

    fn compile_statement(&mut self, body: &[Statement]) {
        for stmt in body {
            match stmt {
                Statement::Assign { name, value } => {
                    self.compile_assignment(name, value);
                },
                Statement::While { condition, body } => {
                    self.compile_while(condition, body);
                }
                Statement::If { condition, then_body, else_body } => {
                    self.compile_expression(condition);
                    self.assem.push("    cmp rax, 0".into());

                    let end_label = self.new_label("endif");
                    let else_label_opt = else_body.as_ref().map(|_| self.new_label("else"));

                    if let Some(ref else_label) = else_label_opt {
                        self.assem.push(format!("    je {}", else_label));
                    } else {
                        self.assem.push(format!("    je {}", end_label));
                    }
                    self.compile_statement(&then_body);
                    if let Some(else_body) = else_body {
                        self.assem.push(format!("    jmp {}", end_label));
                        self.assem.push(format!("{}:", else_label_opt.unwrap()));
                        self.compile_statement(else_body);
                    }
                    self.assem.push(format!("{}:", end_label));
                },
                Statement::FunctionCall { name, args } => {
                    self.assem.push(format!("    call _{}", name));
                    // TODO FIX THIS
                    for arg in args {
                        match arg {
                            Expression::Integer(int) => {
                                self.assem.push(format!("    mov rsi, {}", int));
                            }
                            Expression::Variable(var) => {
                                if let Some(var_offset) = self.offset_map.get(var) {
                                    self.assem.push(format!("    mov rsi, [rbp - {}]", var_offset));
                                } else {
                                    panic!("Variable {} is not defined", var);
                                }
                            }
                            Expression::FunctionArg(arg) => {
                                if let Some(var_offset) = self.offset_map.get(arg) {
                                    self.assem.push(format!("    mov rsi, [rbp - {}]", var_offset));
                                } else {
                                    panic!("Variable {} is not defined", arg);
                                }
                            }
                            _ => {}
                        }
                    }

                },

                

                Statement::Print ( value ) => {
                    self.compile_print(value);
                }
                _ => {}
            }
        }
    }

    fn compile_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Integer(i) => {
                self.assem.push(format!("    mov rax, {}", i));
            }
            Expression::Variable(var) => {
                if let Some(offset) = self.offset_map.get(var) {
                    self.assem.push(format!("    mov rax, [rbp - {}]", offset));
                } else {
                    panic!("Variable {} not defined", var);
                }
            }
            Expression::StringLiteral(s) =>  {
                // Register string literal and get its label.
                let label = self.register_string_literal(s);
                // Load the address of the string literal into RAX.
                self.assem.push(format!("    lea rax, [rel {}]", label));
            },
            Expression::BinaryOp { left, op, right } => {
                // First, compile the left side:
                self.compile_expression(left);
                // Save left operand on the stack:
                self.assem.push("    push rax".into());
                // Then, compile the right side:
                self.compile_expression(right);
                // Retrieve left operand from the stack into rcx:
                self.assem.push("    pop rcx".into());

                // Now, perform the operation:
                match op {
                    crate::ast::BinaryOperator::Add => {
                        self.assem.push("    add rax, rcx".into());
                    }
                    crate::ast::BinaryOperator::Sub => {
                        self.assem.push("    sub rcx, rax".into());
                        self.assem.push("    mov rax, rcx".into());
                    }
                    crate::ast::BinaryOperator::Mul => {
                        self.assem.push("    imul rax, rcx".into());
                    }
                    crate::ast::BinaryOperator::Div => {
                        self.assem.push("    mov rbx, rax".into());  // Save right operand
                        self.assem.push("    mov rax, rcx".into());    // Move left operand into RAX
                        self.assem.push("    mov rdx, 0".into());        // Clear rdx
                        self.assem.push("    idiv rbx".into());
                    },
                    crate::ast::BinaryOperator::Eq => {
                        self.assem.push("    cmp rax, rcx".into());
                        self.assem.push("    sete al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                    crate::ast::BinaryOperator::NEq => {
                        self.assem.push("    cmp rax, rcx".into());
                        self.assem.push("    setne al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                    crate::ast::BinaryOperator::Lt => {
                        self.assem.push("    cmp rax, rcx".into());
                        self.assem.push("    setl al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                    crate::ast::BinaryOperator::Gt => {
                        self.assem.push("    cmp rax, rcx".into());
                        self.assem.push("    setg al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                }
            }
            _ => panic!("Unhandled expression type"),
        }
    }

    fn compile_assignment(&mut self, name: &String, value: &crate::ast::Expression) {
        let offset = if let Some(&offset) = self.offset_map.get(name) {
            offset
        } else {
            let off = self.var_offset;
            self.offset_map.insert(name.clone(), off);
            self.var_offset += 8;
            off
        };
        self.compile_expression(value);
        self.assem.push(format!("    mov [rbp - {}], rax", offset));
    }

    fn compile_params(&mut self, params: &[crate::ast::Expression]) {
        
    }

    fn compile_print(&mut self, value: &crate::ast::Expression) {
        match value {
            Expression::Integer(int) => {
                self.assem.push(format!("   mov rsi, {}", int));
                self.assem.push("    lea rdi, [rel fmt]".to_string());

            }
            Expression::StringLiteral(s) => {
                let label = self.register_string_literal(s);
                self.assem.push(format!("    lea rsi, [rel {}]", label));
                self.assem.push("    lea rdi, [rel fmt_str]".to_string()); // fmt_str for string, e.g., "%s\n"
            }
            Expression::Variable(var) => {
                if let Some(var_offset) = self.offset_map.get(var) {
                    self.assem.push(format!("    mov rsi, [rbp - {}]", var_offset));
                    self.assem.push("    lea rdi, [rel fmt]".to_string());
                } else {
                    panic!("Variable {} is not defined", var);
                }
            }
            _ => {}
        }

        self.assem.push("    mov rax, 0".to_string());
        self.assem.push("    call _printf".to_string());
    }

    fn compile_while(&mut self, condition: &Expression, body: &[Statement]) {
        let start_label = self.new_label("while_start");
        let end_label = self.new_label("while_end");

        self.assem.push(format!("{}:", start_label));
        self.compile_expression(condition);
        self.assem.push("    cmp rax, 0".into());
        self.assem.push(format!("    je {}", end_label));

        for stmt in body {
            self.compile_statement(&[stmt.clone()]);
        }

        self.assem.push(format!("    jmp {}", start_label));
        self.assem.push(format!("{}:", end_label));
    }
}
