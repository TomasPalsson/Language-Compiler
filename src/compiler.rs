use std::collections::{HashMap, HashSet};

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
    epilogue_label: String,
    string_vars: HashSet<String>,
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
            epilogue_label: String::new(),
            string_vars: HashSet::new(),
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
        data.push("extern _bonk_http_fetch".into());
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
            // Save outer scope
            let saved_offset_map = std::mem::take(&mut self.offset_map);
            let saved_var_offset = self.var_offset;
            let saved_epilogue_label = std::mem::take(&mut self.epilogue_label);
            let saved_string_vars = std::mem::take(&mut self.string_vars);
            self.var_offset = 8;
            self.epilogue_label = self.new_label("epilogue");

            if name != "main" {
                self.assem.push(format!("global _{}", name));
            }
            // Prologue
            self.assem.push(format!("_{}:", name));
            self.assem.push("    push rbp".into());
            self.assem.push("    mov rbp, rsp".into());
            // Placeholder for stack reservation — patched after body compilation
            let sub_rsp_idx = self.assem.len();
            self.assem.push("    sub rsp, 0".into());

            // Spill params to stack slots
            self.compile_params(params);

            self.compile_statement(body);

            // Patch frame size (round up to 16-byte alignment)
            let frame_size = ((self.var_offset as usize + 15) / 16) * 16;
            self.assem[sub_rsp_idx] = format!("    sub rsp, {}", frame_size);

            // Epilogue — default return 0, then shared cleanup
            self.assem.push("    mov rax, 0".into());
            self.assem.push(format!("{}:", self.epilogue_label));
            self.assem.push("    mov rsp, rbp".into());
            self.assem.push("    pop rbp".into());
            self.assem.push("    ret".into());

            // Restore outer scope
            self.offset_map = saved_offset_map;
            self.var_offset = saved_var_offset;
            self.epilogue_label = saved_epilogue_label;
            self.string_vars = saved_string_vars;
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
                    let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                    // Evaluate each arg and push result onto stack
                    for arg in args {
                        self.compile_expression(arg);
                        self.assem.push("    push rax".into());
                    }
                    // Pop into registers in reverse order
                    for i in (0..args.len()).rev() {
                        self.assem.push(format!("    pop {}", arg_regs[i]));
                    }
                    // ABI: al = 0 (no floating-point args)
                    self.assem.push("    mov rax, 0".into());
                    self.assem.push(format!("    call _{}", name));
                },

                

                Statement::Print ( value ) => {
                    self.compile_print(value);
                }
                Statement::Send(expr) => {
                    self.compile_expression(expr);
                    self.assem.push(format!("    jmp {}", self.epilogue_label));
                }
                Statement::Fetch { method, url, body } => {
                    self.compile_fetch(method, url, body.as_ref());
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
                        self.assem.push("    cmp rcx, rax".into());
                        self.assem.push("    setl al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                    crate::ast::BinaryOperator::LtEq => {
                        self.assem.push("    cmp rcx, rax".into());
                        self.assem.push("    setle al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                    crate::ast::BinaryOperator::Gt => {
                        self.assem.push("    cmp rcx, rax".into());
                        self.assem.push("    setg al".into());
                        self.assem.push("    movzx rax, al".into());
                    }
                }
            }
            Expression::FunctionArg(name) => {
                if let Some(offset) = self.offset_map.get(name) {
                    self.assem.push(format!("    mov rax, [rbp - {}]", offset));
                } else {
                    panic!("Function argument {} not defined", name);
                }
            }
            Expression::FunctionCall { name, args } => {
                let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                for arg in args {
                    self.compile_expression(arg);
                    self.assem.push("    push rax".into());
                }
                for i in (0..args.len()).rev() {
                    self.assem.push(format!("    pop {}", arg_regs[i]));
                }
                self.assem.push("    mov rax, 0".into());
                self.assem.push(format!("    call _{}", name));
            }
            Expression::Fetch { method, url, body } => {
                self.compile_fetch(method, url, body.as_deref());
            }
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
        // Track whether this variable holds a string value
        if matches!(value, Expression::StringLiteral(_) | Expression::Fetch { .. }) {
            self.string_vars.insert(name.clone());
        } else {
            self.string_vars.remove(name);
        }
        self.compile_expression(value);
        self.assem.push(format!("    mov [rbp - {}], rax", offset));
    }

    fn compile_params(&mut self, params: &[crate::ast::Expression]) {
        let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
        for (i, param) in params.iter().enumerate() {
            if let Expression::FunctionArg(name) = param {
                let offset = self.var_offset;
                self.offset_map.insert(name.clone(), offset);
                self.var_offset += 8;
                self.assem.push(format!("    mov [rbp - {}], {}", offset, regs[i]));
            }
        }
    }

    fn compile_print(&mut self, value: &crate::ast::Expression) {
        let fmt_label = match value {
            Expression::StringLiteral(_) | Expression::Fetch { .. } => "fmt_str",
            Expression::Variable(name) if self.string_vars.contains(name) => "fmt_str",
            _ => "fmt",
        };
        self.compile_expression(value);
        self.assem.push("    mov rsi, rax".into());
        self.assem.push(format!("    lea rdi, [rel {}]", fmt_label));
        self.assem.push("    mov rax, 0".into());
        self.assem.push("    call _printf".into());
    }

    fn compile_fetch(&mut self, method: &Expression, url: &Expression, body: Option<&Expression>) {
        // Evaluate args left-to-right, push onto stack
        self.compile_expression(method);
        self.assem.push("    push rax".into());
        self.compile_expression(url);
        self.assem.push("    push rax".into());
        if let Some(body_expr) = body {
            self.compile_expression(body_expr);
            self.assem.push("    push rax".into());
        } else {
            self.assem.push("    push 0".into()); // NULL body
        }
        // Pop into argument registers: rdi=method, rsi=url, rdx=body
        self.assem.push("    pop rdx".into());
        self.assem.push("    pop rsi".into());
        self.assem.push("    pop rdi".into());
        self.assem.push("    mov rax, 0".into());
        self.assem.push("    call _bonk_http_fetch".into());
        // Result (response string pointer) is in rax
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
