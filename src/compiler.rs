use std::collections::HashMap;

use crate::ast::{Statement, Expression, BinaryOperator};
use std::slice::Iter;
use std::{self, iter::Peekable};

pub struct Compiler {
    offset_map: HashMap<String, i32>,
    value_map: HashMap<String, i32>,
    functions: Vec<String>,
    assem: Vec<String>,
    var_offset: i32,
}


impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            offset_map: HashMap::new(),
            value_map: HashMap::new(),
            functions: Vec::new(),
            assem: Vec::new(),
            var_offset: 8,
        }
    }
    pub fn compile(&mut self, ast: Vec<Statement>) -> Vec<String> {
        let mut iter = ast.iter().peekable();
        self.compiler(&mut iter);
        self.assem.clone()
    }

    fn compiler(&mut self, iter: &mut Peekable<Iter<Statement>>) {
        self.emit_data();
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


    fn emit_data(&mut self) {
        self.assem.push("extern _printf".into());
        self.assem.push("section .rodata".into());
        self.assem.push("fmt: db \"%ld\", 10, 0".to_string());

        // main and text section
        self.assem.push("global _main".into());
        self.assem.push("section .text".into());
    }

    fn compile_function(&mut self, iter: &mut Peekable<Iter<Statement>>) {
        if let Some(Statement::Function { name, params, body }) = iter.peek() {
            if name != "main" {
                self.assem.push(format!("global _{}", name));
            }
            self.assem.push(format!("_{}:", name));
            self.assem.push(";   ; prologue".into());
            self.assem.push("    push rbp".into());
            self.assem.push("    mov rbp, rsp".into());
            // reserve space 
            self.assem.push("; Reserving space".into());
            self.assem.push("    sub rsp, 24".into());
            self.assem.push(";   ; BODY".into());
            self.compile_statement(body);
            self.assem.push("    mov rsp, rbp".into());
            // TODO  temp 
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
                }
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
                    }
                }
            }
            _ => panic!("Unhandled expression type"),
        }
    }

    fn compile_assignment(&mut self, name: &String, value: &crate::ast::Expression) {
        let offset = self.var_offset;
        self.offset_map.insert(name.clone(), offset);
        self.var_offset += 8;
        self.compile_expression(value);
        self.assem.push(format!("    mov [rbp - {}], rax", offset));
    }

    fn compile_params(&mut self, params: &[crate::ast::Expression]) {
        
    }

    fn compile_print(&mut self, value: &crate::ast::Expression) {
        match value {
            crate::ast::Expression::Integer(int) => {
                self.assem.push(format!("   mov rsi, {}", int));
            }
            crate::ast::Expression::Variable(var) => {
                if let Some(var_offset) = self.offset_map.get(var) {
                    self.assem.push(format!("    mov rsi, [rbp - {}]", var_offset));
                } else {
                    panic!("Variable {} is not defined", var);
                }
            }
            _ => {}
        }

        self.assem.push("    lea rdi, [rel fmt]".to_string());
        self.assem.push("    mov rax, 0".to_string());
        self.assem.push("    call _printf".to_string());
    }
}
