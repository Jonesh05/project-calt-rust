use leptos::*;
use leptos::logging::log;
use web_sys::MouseEvent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Calculator {
    display: String,
    current_number: String,
    operation: Option<char>,
    previous_number: Option<f64>,
    history: Vec<String>,
}



impl Calculator {
    fn new() -> Self {
        Calculator {
            display: String::from("0"),
            current_number: String::new(),
            operation: None,
            previous_number: None,
            history: Vec::new(),
        }
    }

    fn push(&mut self, value: &str) -> Result<(), String> {
        match value {
            "+" | "-" | "*" | "/" => {
                if !self.current_number.is_empty() {
                    self.operation = Some(value.chars().next().unwrap());
                    self.previous_number = Some(self.current_number.parse().unwrap());
                    self.current_number.clear();
                }
            }
            "=" => {
                if let (Some(prev), Some(op)) = (self.previous_number, self.operation) {
                    if let Ok(current) = self.current_number.parse::<f64>() {
                        let result = match op {
                            '+' => prev + current,
                            '-' => prev - current,
                            '*' => prev * current,
                            '/' => {
                                if current == 0.0 {
                                    return Err("División por cero".to_string());
                                }
                                prev / current
                            }
                            _ => return Err("Operación inválida".to_string()),
                        };
                        
                        let operation = format!("{} {} {} = {}", prev, op, current, result);
                        self.history.push(operation);
                        
                        self.current_number = result.to_string();
                        self.previous_number = None;
                        self.operation = None;
                    }
                }
            }
            "ac" => self.reset(),
            "<" => self.undo(),
            _ => self.current_number.push_str(value),
        }
        
        self.update_display();
        Ok(())
    }

    fn reset(&mut self) {
        self.display = String::from("0");
        self.current_number.clear();
        self.operation = None;
        self.previous_number = None;
    }

    fn undo(&mut self) {
        if !self.current_number.is_empty() {
            self.current_number.pop();
            self.update_display();
        }
    }

    fn update_display(&mut self) {
        self.display = if self.current_number.is_empty() {
            String::from("0")
        } else {
            self.current_number.clone()
        };
    }

    fn get_display(&self) -> String {
        self.display.clone()
    }

    fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
}

#[component]
fn App() -> impl IntoView {
    let calculator = Rc::new(RefCell::new(Calculator::new()));
    let (display, set_display) = create_signal(String::from("0"));
    let (history, set_history) = create_signal(Vec::<String>::new());
    
    let calculator_clone = calculator.clone();
    let on_clicked = move |ev: MouseEvent| {
        let value = event_target_value(&ev);
        log!("* clicked value [{}]", value);
        
        let mut calc = calculator_clone.borrow_mut();
        if let Ok(_) = calc.push(&value) {
            set_display.set(calc.get_display());
            set_history.set(calc.get_history());
        }
    };

    view! {
        <div class="calculator">
            <div class="display">
                {move || display.get()}
            </div>
    
            <div class="keypad">
                // Botones de la calculadora
                <button on:click=on_clicked.clone() value="7">"7"</button>
                <button on:click=on_clicked.clone() value="8">"8"</button>
                <button on:click=on_clicked.clone() value="9">"9"</button>
                <button on:click=on_clicked.clone() value="/">"÷"</button>
    
                <button on:click=on_clicked.clone() value="4">"4"</button>
                <button on:click=on_clicked.clone() value="5">"5"</button>
                <button on:click=on_clicked.clone() value="6">"6"</button>
                <button on:click=on_clicked.clone() value="*">"×"</button>
    
                <button on:click=on_clicked.clone() value="1">"1"</button>
                <button on:click=on_clicked.clone() value="2">"2"</button>
                <button on:click=on_clicked.clone() value="3">"3"</button>
                <button on:click=on_clicked.clone() value="-">"-"</button>
    
                <button on:click=on_clicked.clone() value="0">"0"</button>
                <button on:click=on_clicked.clone() value=".">"."</button>
                <button on:click=on_clicked.clone() value="=">"="</button>
                <button on:click=on_clicked.clone() value="+">"+"</button>
            </div>
    
            <div class="control-buttons">
                <button on:click=on_clicked.clone() value="ac" class="clear">"AC"</button>
                <button on:click=on_clicked.clone() value="<" class="backspace">"⬅"</button>
            </div>
    
            <div class="history">
                <h3>"Historial"</h3>
                {move || {
                    let calculator_ref = calculator.clone();  // Clonamos calculator fuera del mapa
                    history.get().into_iter().map(move |operation| {
                        let calculator_clone = calculator_ref.clone();  // Clonamos dentro del mapa para evitar mover `calculator`
                        view! {
                            <div class="history-item">
                                {operation.clone()}
                                <button 
                                    on:click=move |_| {
                                        let result = operation.split("=").last()
                                            .unwrap_or("0")
                                            .trim()
                                            .to_string();
                                        let mut calc = calculator_clone.borrow_mut();
                                        calc.current_number = result;
                                        calc.update_display();
                                        set_display.set(calc.get_display());
                                    }
                                >
                                    "Usar"
                                </button>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
    
    
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
