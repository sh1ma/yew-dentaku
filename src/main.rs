use yew::prelude::*;
use rpn::{FormulaElement, Operator, calculate_rpn, convert_infix_to_rpn};
use std::rc::Rc;
use wasm_logger;
use log;


mod rpn;
mod dentaku_core;

use dentaku_core::dentaku_input::DentakuInputState;
use dentaku_core::model::InputtableNumber;

#[derive(Clone, Debug)]
pub enum DentakuStateAction {
    UpdateDentakuInput(InputtableNumber),
    PushFormulaElementPair(FormulaElement, FormulaElement),
    PushRightAndCalculate(FormulaElement),
    InputDecimalPoint,
    Clear,
    AllClear
}

#[derive(Clone, Debug, PartialEq)]
struct DentakuState {
    formula: Vec<FormulaElement>,
    dentaku_input: DentakuInputState
}

impl Default for DentakuState {
    fn default() -> Self {
        Self {
            formula: Vec::new(),
            dentaku_input: DentakuInputState::default()
        }
    }
}


impl Reducible for DentakuState {
    /// Reducer Action Type
    type Action = DentakuStateAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            DentakuStateAction::UpdateDentakuInput(value) => {
                Self {
                    dentaku_input: self.dentaku_input.try_add_digit(value).expect("unexcpted error"),
                    formula: self.formula.clone()
                }.into()
            }
            DentakuStateAction::PushFormulaElementPair(operand, operator) => {
                let mut current_formula = self.formula.clone();

                current_formula.push(operand);
                current_formula.push(operator);

                Self {
                    dentaku_input: self.dentaku_input.clone()
                    .update_value(String::new())
                    ,
                    formula: current_formula
                }.into()
            }
            DentakuStateAction::PushRightAndCalculate(operand) => {
                let mut current_formula = self.formula.clone();

                current_formula.push(operand);

                let formula = current_formula.clone();

                let result = calculate_rpn(convert_infix_to_rpn(current_formula)).expect("unexcepted error");



                Self {
                    dentaku_input: self.dentaku_input.clone()
                    .update_value(result.to_string()),
                    formula: Vec::new()
                }.into()
            }
            DentakuStateAction::InputDecimalPoint => {
                let updated_dentaku_input = match self.dentaku_input.try_add_decimal_point() {
                    Ok(state) => state,
                    Err(error) => self.dentaku_input.clone()
                };

                Self {
                    dentaku_input: updated_dentaku_input,
                    formula: self.formula.clone()
                }
            }.into(),
            DentakuStateAction::Clear => Self {
                dentaku_input: DentakuInputState::default(),
                formula: self.formula.clone()
            }.into(),
            DentakuStateAction::AllClear => Self::default().into()
        }
    }
}
type DentakuContext = UseReducerHandle<DentakuState>;

#[function_component(App)]
fn app() -> Html {
    let ctx = use_reducer(DentakuState::default);

    html! {
        <div class="app">
            <ContextProvider<DentakuContext> context={ctx}>
                <Dentaku />
            </ContextProvider<DentakuContext>>
        </div>
    }
}

#[function_component(Dentaku)]
fn dentaku() -> Html {
    html! {
        <div class="dentaku-wrapper">
            <DentakuResult />
            <DentakuInputBoard />
        </div>
    }
}

#[function_component(DentakuResult)]
fn dentaku_result() -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");

    html! {
        <div class="dentaku-result">
            <span class="result-text">{ state.dentaku_input.to_string() }</span>
        </div>
    }
}

#[function_component(DentakuInputBoard)]
fn dentaku_input_board() -> Html {
    html! {
        <div class="dentaku-input">
            <ClearButton />
            // <div class="button-container"><button class="special-button">{"AC"}</button></div>
            <div class="button-container"><button class="special-button">{"x(-1)"}</button></div>
            <div class="button-container"><button class="special-button">{"%"}</button></div>
            // <div class="button-container"><button class="special-button">{"÷"}</button></div>
            <OperatorButton text={"÷".to_string()} operator={Operator::Div} />
            // <div class="button-container"><button class="number-button">{"7"}</button></div>
            <NumberButton number={7} />
            <NumberButton number={8} />
            <NumberButton number={9} />
            <OperatorButton text={"x".to_string()} operator={Operator::Mul} />
            <NumberButton number={4} />
            <NumberButton number={5} />
            <NumberButton number={6} />
            <OperatorButton text={"-".to_string()} operator={Operator::Sub} />
            <NumberButton number={1} />
            <NumberButton number={2} />
            <NumberButton number={3} />
            <OperatorButton text={"+".to_string()} operator={Operator::Add} />
            <div class="zero-button-container"><NumberZeroButton number={0} /></div>
            <DotButton />
            // <div class="button-container"><button class="special-button">{"="}</button></div>
            <EqualButton />
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct NumberButtonProps {
    pub number: i32
}

#[function_component(NumberButton)]
fn number_button(props: &NumberButtonProps) -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");

    let number = props.number;

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(DentakuStateAction::UpdateDentakuInput(InputtableNumber { value: number })))
    };


    html!(
        <div class="button-container">
            <button class="number-button" onclick={onclick}>{ props.number }</button>
        </div>
    )
}

#[function_component(NumberZeroButton)]
fn number_zero_button(props: &NumberButtonProps) -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");

    let number = props.number;

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(DentakuStateAction::UpdateDentakuInput(InputtableNumber { value: number })))
    };


    html!(
        <div class="zero-button-container">
            <button class="zero-button" onclick={onclick}>{ props.number }</button>
        </div>
    )
}

#[function_component(DotButton)]
fn dot_button() -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");

    let onclick = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| state.dispatch(DentakuStateAction::InputDecimalPoint))
    };

    html!(
        <div class="button-container">
            <button class="number-button" onclick={onclick}>{ "." }</button>
        </div>
    )
}

#[function_component(EqualButton)]
fn equal_button() -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");

    let onclick = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| state.dispatch(DentakuStateAction::PushRightAndCalculate(FormulaElement::Operand(state.dentaku_input.to_f32()))))
    };

    html!(
        <div class="button-container">
            <button class="special-button" onclick={onclick}>{ "=" }</button>
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
struct OperatorButtonProps {
    pub text: String,
    pub operator: Operator
}

#[function_component(OperatorButton)]
fn operator_button(props: &OperatorButtonProps) -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");

    let onclick = {
        let state = state.clone();
        // state.dispatch(DentakuStateAction::PushFormulaElement(FormulaElement::Operand(state.dentaku_input.to_f32())));
        
        let op = props.operator.clone();

        Callback::from(move |_: MouseEvent| 
            state.dispatch(DentakuStateAction::PushFormulaElementPair(FormulaElement::Operand(state.dentaku_input.to_f32()), FormulaElement::Operator(op)))
        )
    };

    let button_text: &str = &props.text;

    html!(
        <div class="button-container">
            <button class="operator-button" onclick={onclick}>{ button_text }</button>
        </div>
    )
}


#[derive(PartialEq)]
enum ClearButtonType {
    AllClear,
    Clear
}

#[function_component(ClearButton)]
fn clear_button() -> Html {
    let state = use_context::<DentakuContext>().expect("no ctx found");
    let button_type = if state.dentaku_input.is_empty() { ClearButtonType::AllClear } else { ClearButtonType::Clear };
    let button_name = if button_type == ClearButtonType::AllClear {"AC"} else {"C"};
    let onclick = {
        let state = state.clone();
        if button_type == ClearButtonType::AllClear {
            Callback::from(move |_: MouseEvent| state.dispatch(DentakuStateAction::AllClear))
        } else {
            Callback::from(move |_: MouseEvent| state.dispatch(DentakuStateAction::Clear))
        }
    };

    html! {
        <div class="button-container">
            <button class="special-button" onclick={onclick}>{ button_name }</button>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}