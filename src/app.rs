use yew::prelude::*;
use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::builtin_words::select;
mod resouces;
use resouces::_dmode_vavid_check;
use std::collections::HashMap;
mod judge_yew;
use stylist::Style;

pub struct Model {
    show_menu: bool,
    is_difficult:bool,
    seed:String,
    day:String,
    inputs: Vec<Vec<String>>,
    current_row: usize,
    all_submitted : bool,
    colors: Vec<Vec<String>>,
    answer: String,
    result: String,
    input : String,
    all_completed : bool,
    alphabet_color: String,
    char_color : HashMap<char , char>,
    total_round:i32,
    total_success:i32,
    rounds:i32,
    average_round:f32,
    words:HashMap<String , i32>,
    total_success_rounds:i32,
    flip_flags: Vec<bool>,
}

pub enum Msg {
    ToggleMenu,
    SelectValue(String),
    ClickOutside,
    _Clickwithnothing,
    DifficultCheck,
    UpdateSeed(String),
    UpdateDay(String),
    UpdateInput(usize,usize,String),
    EnableNextRow,
    FocusNext((usize, usize)),
    ClearRow(usize),
    FocusPrevious((usize, usize)),
    Reset,
    TriggerFlip(usize), // 添加一个消息来触发翻转动画
    ResetFlip(usize), // 添加一个消息来重置翻转标志
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let inputs = vec![vec![String::new(); 5]; 6];
        let colors = vec![vec!["".to_string(); 5]; 6];
        let alphabet_color = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();

        let onclick = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
            link.send_message(Msg::ClickOutside);
        }) as Box<dyn FnMut(_)>);

        body.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref()).unwrap();
        onclick.forget();

        Self {
            show_menu: false,
            is_difficult:false,
            seed:"114514".to_string(),
            day:"810".to_string(),
            inputs,
            current_row: 0,
            all_submitted: false,
            colors,
            answer:String::from("TITAN"),
            result: String::from(""),
            input: String::from(""),
            all_completed : false,
            alphabet_color,
            char_color : HashMap::new(),
            total_round:0,
            total_success:0,
            total_success_rounds:0,
            rounds:0,
            words:HashMap::new(),
            average_round:0.0,
            flip_flags: vec![false; 6],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.show_menu = !self.show_menu;
            }
            Msg::_Clickwithnothing=>{       
            }
            Msg::SelectValue(value) => {
                self.show_menu = false;
                info!("Selected value: {}", value);
            }
            Msg::ClickOutside => {
                if self.show_menu {
                    self.show_menu = false;
                    return true;
                }
            }
            Msg::DifficultCheck =>{
                self.is_difficult = !self.is_difficult;       
            }
            Msg::UpdateSeed(value) =>{
                self.seed = value.parse().unwrap();
                let mut seed = 114514;
                match self.seed.parse::<u64>() {
                    Ok(value) => {
                        seed = value;
                    },
                    Err(_e) => {
                        seed = 114514;
                    }
                }
                let mut day = 810;
                match self.day.parse::<usize>(){
                    Ok(value) =>{
                        day = value;
                    },
                    Err(_e) =>{
                        day = 810;
                    }
                }
                self.answer = resouces::get_useable_word_default(day, seed);
            }
            Msg::UpdateDay(value) =>{
                self.day = value.parse().unwrap();
                let mut seed = 114514;
                match self.seed.parse::<u64>() {
                    Ok(value) => {
                        seed = value;
                    },
                    Err(_e) => {
                        seed = 114514;
                    }
                }
                let mut day = 810;
                match self.day.parse::<usize>(){
                    Ok(value) =>{
                        day = value;
                    },
                    Err(_e) =>{
                        day = 810;
                    }
                }
                self.answer = resouces::get_useable_word_default(day, seed);
            }
            Msg::UpdateInput(row, col, value) => {
                let value = value.to_uppercase();  // 转换为大写
                if row == self.current_row && !self.all_submitted {
                    if let Some(input_row) = self.inputs.get_mut(row) {
                        if let Some(input) = input_row.get_mut(col) {
                            let is_empty = value.is_empty();
                            *input = value.clone();
                            if is_empty && col > 0 {
                                ctx.link().send_message(Msg::FocusPrevious((row, col - 1)));
                            } else if !is_empty && value.len() == 1 && col < 4 {
                                ctx.link().send_message(Msg::FocusNext((row, col + 1)));
                            }
                        }
                    }
                }
            }
            Msg::EnableNextRow => {
                // 进行合法性检查
                if !self.validate_row(self.current_row) {
                    return false;
                }
                let (result, input , alphabet_result , char_color) = judge_yew::crate_judge_yew::judge(self.inputs[self.current_row].clone() , self.answer.clone().as_str() , self.char_color.clone());
                if self.is_difficult{
                    if !_dmode_vavid_check(self.input.as_str(), &input, &self.result){
                        return false;
                    }
                }
                let value = self.words.entry(input.clone()).or_insert(0);
                    *value += 1;
                self.result = result.clone();
                self.input = input.clone();
                self.char_color = char_color.clone();
                let mut all_green = true;
                for (col, color) in result.chars().enumerate() {
                    self.colors[self.current_row][col] = match color {
                        'G' => "green".to_string(),
                        'Y' => "#CCCC00".to_string(),
                        'R' => "red".to_string(),
                        _ => "white".to_string(),
                    };
                    if color != 'G' {
                        all_green = false;
                    }
                }
                self.rounds += 1;
                if all_green {
                    self.total_success += 1;
                    self.total_round += 1;
                    self.total_success_rounds += self.rounds;
                    self.words = resouces::hash_map_sort(self.words.clone());
                    self.average_round = self.total_success_rounds as f32 / self.total_success as f32;
                    self.all_completed = true;
                    self.rounds = 0;
                    self.day = (self.day.parse::<usize>().unwrap() + 1).to_string();
                    self.answer = resouces::get_useable_word_default(self.day.parse::<usize>().unwrap(), self.seed.parse::<u64>().unwrap());
                    self.result = String::from("");
                    self.input = String::from("");
                    self.alphabet_color= "XXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
                    let link = ctx.link().clone();
                    let handle = move || {
                        // 显示警告框
                        web_sys::window().unwrap().alert_with_message("Congratulations! The game will reset.").unwrap();
                        // 使用定时器在警告框关闭后进行重置操作
                        let link_clone = link.clone();
                        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                            Closure::once_into_js(move || {
                                link_clone.send_message(Msg::Reset);
                            }).as_ref().unchecked_ref(), 
                            0 // 0毫秒延迟，表示尽快执行
                        ).unwrap();
                    };
                    let _ = wasm_bindgen_futures::spawn_local(async move {
                        gloo::timers::future::TimeoutFuture::new(100).await; // 500ms 延迟
                        handle();
                    });
                }else if !all_green && self.current_row == 5{
                    self.total_round += 1;
                    self.rounds = 0;
                    self.day = (self.day.parse::<usize>().unwrap() + 1).to_string();
                    let answer_before = self.answer.clone();
                    self.answer = resouces::get_useable_word_default(self.day.parse::<usize>().unwrap(), self.seed.parse::<u64>().unwrap());
                    self.result = String::from("");
                    self.input = String::from("");
                    self.alphabet_color= "XXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
                    let link = ctx.link().clone();
                    let handle = move || {
                        web_sys::window().unwrap().alert_with_message(format!("ざこお兄ちゃん～～ The answer is {}.The game will reset.", answer_before).as_str()).unwrap();
                        let link_clone = link.clone();
                        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                            Closure::once_into_js(move || {
                                link_clone.send_message(Msg::Reset);
                            }).as_ref().unchecked_ref(), 
                            0 // 0毫秒延迟，表示尽快执行
                        ).unwrap();
                    };
                    self.current_row = 0;
                    self.all_completed = false;
                    let _ = wasm_bindgen_futures::spawn_local(async move {
                        gloo::timers::future::TimeoutFuture::new(100).await; // 500ms 延迟
                        handle();
                    });
                }
                self.alphabet_color = alphabet_result.clone();
                if !self.all_submitted && self.current_row < self.inputs.len() && self.inputs[self.current_row].iter().all(|s| !s.is_empty()) {
                    ctx.link().send_message(Msg::TriggerFlip(self.current_row));
                    self.current_row += 1;
                    if self.current_row == self.inputs.len() {
                        self.all_submitted= true;
                    }
                }
            }
            Msg::FocusNext((row, col)) => {
                let input_id = format!("input-{}-{}", row, col);
                if let Some(document) = web_sys::window().and_then(|win| win.document()) {
                    if let Some(element) = document.get_element_by_id(&input_id) {
                        if let Some(input) = element.dyn_into::<HtmlInputElement>().ok() {
                            input.focus().unwrap();
                        }
                    }
                }
            }
            Msg::FocusPrevious((row, col)) => {
                let input_id = format!("input-{}-{}", row, col);
                if let Some(document) = web_sys::window().and_then(|win| win.document()) {
                    if let Some(element) = document.get_element_by_id(&input_id) {
                        if let Some(input) = element.dyn_into::<HtmlInputElement>().ok() {
                            input.focus().unwrap();
                        }
                    }
                }
            }
            Msg::ClearRow(row)=>{
                if let Some(input_row) = self.inputs.get_mut(row) {
                    for input in input_row.iter_mut() {
                        *input = String::new();
                    }
                }
            }
            Msg::Reset => {
                self.alphabet_color = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
                self.all_completed = false;
                self.inputs = vec![vec![String::new(); 5]; 6];
                self.colors = vec![vec!["".to_string(); 5]; 6];
                self.current_row = 0;
                self.all_completed = false;
                self.char_color = HashMap::new();
                self.flip_flags = vec![false; 6];
            }
            Msg::TriggerFlip(row) => {
                self.flip_flags[row] = true; // 启用翻转标志
                let link = ctx.link().clone();
                // 使用定时器在动画结束后禁用翻转标志
                let handle = move || {
                    link.send_message(Msg::ResetFlip(row));
                };
                let _ = wasm_bindgen_futures::spawn_local(async move {
                    gloo::timers::future::TimeoutFuture::new(600).await; // 动画时长 600ms
                    handle();
                });
            }
            Msg::ResetFlip(row) =>{
                self.flip_flags[row] = false;
            }
        }
        true
    }

    fn changed(&mut self, _: &Context<Self>, _: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu_style = if self.show_menu {
            "opacity: 1; visibility: visible; transition: opacity 0.5s ease-in-out; \
             width: 12.5rem; border-radius: 0.3125rem; background: #f9f9f9; box-shadow: 0 0.25rem 0.5rem rgba(0, 0, 0, 0.1);"
        } else {
            "opacity: 0; visibility: hidden; transition: opacity 0.5s ease-in-out; \
             width: 12.5rem; border-radius: 0.3125rem; background: #f9f9f9; box-shadow: 0 0.25rem 0.5rem rgba(0, 0, 0, 0.1);"
        };
    
        html! {
            <div>
                //顶部中心图片
                <div style="position: fixed; top: 0; left: 50%; transform: translateX(-50%); z-index: 2000;">
                    <a href="/wordle/">
                        <img src="https://vip.helloimg.com/i/2024/07/09/668c11135062e.png" alt="Top Center Image" style="width: 12.5rem; height: auto;" />
                        <span style="position: absolute; bottom: 1.25rem; right: -4.6875rem; background-color: rgba(255, 255, 255, 0.7); padding: 0.125rem 0.3125rem; border-radius: 0.1875rem; font-size: 0.875rem; color: #000;">
                            { "𝓑𝔂 𝓡𝓸𝓼𝓲𝓼𝓽" }
                        </span>
                    </a>
                </div>
                <hr style="position: fixed; top: 5rem; left: 0; border: 0; border-top: 0.125rem solid #ccc; width: 100%; z-index: 999;" />
                //菜单按钮和config
                <div style="position: fixed; top: 1.875rem; left: 1.25rem; z-index: 2000;">
                    <button style="background: none; border: none; cursor: pointer;" onclick={ctx.link().callback(|e: MouseEvent| {
                        e.stop_propagation();
                        Msg::ToggleMenu
                    })}>
                        <img src="https://vip.helloimg.com/i/2024/07/08/668be3bc35970.png" alt="Settings" style="width: 2.8125rem; height: 1.5rem;" />
                    </button>
                    <ul style={menu_style}>
                        <li style="padding: 0.5rem; margin-bottom: 0.1875rem;cursor: pointer;" onclick={ctx.link().callback(|e: MouseEvent| {
                            e.stop_propagation();
                            Msg::SelectValue("Difficult: ".into())
                        })}>{ "Difficult " }
                        <button onclick={ctx.link().callback(|e: MouseEvent| {
                            e.stop_propagation();
                            Msg::DifficultCheck
                        })} style="margin-left: 0.3125rem; background: none; border: none; cursor: pointer; font-size: 1.1875rem; margin-top: -0.125rem">
                            { if self.is_difficult { "☑" } else { "☐" } }
                        </button>
                        </li>
                        <li style="padding: 0.5rem; cursor: pointer;">
                            { "Seed: " }
                            <input
                                type="text"
                                value={self.seed.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateSeed(input.value())
                                })}
                                onclick={ctx.link().callback(|e: MouseEvent| {
                                    e.stop_propagation();
                                    Msg::_Clickwithnothing
                                })}
                                style="margin-left: 0.625rem; padding: 0.25rem; font-size: 1rem; width: 6.25rem;"
                            />
                        </li>
                        <li style="padding: 0.5rem; cursor: pointer;" onclick={ctx.link().callback(|e: MouseEvent| {
                            e.stop_propagation();
                            Msg::SelectValue("Day: ".into())
                        })}>{ "Day: " }<input
                            type="text"
                            value={self.day.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateDay(input.value())
                            })}
                            onclick={ctx.link().callback(|e: MouseEvent| {
                                e.stop_propagation();
                                Msg::_Clickwithnothing
                            })}
                            style="margin-left: 0.625rem; padding: 0.25rem; font-size: 1rem; width: 6.25rem;"
                        />
                    </li>
                    </ul>
            </div>
                //主要输入框
            <div style= "margin-top: 30px; padding: 20px; overflow-y: auto;">
                <div style="position: fixed; top: 9.375rem; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; gap: 0.625rem; z-index: 1500;">
                    { self.view_styles() }
                    { (0..6).map(|row| self.view_row(ctx, row)).collect::<Html>() }
                </div>
                <div style="position: fixed; top: 53.125rem; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; gap: 0.625rem; z-index: 1500;">
                    { self.view_keyboard() }
                    <div>
                        <button style="position: relative; bottom: 8.3125rem; left: 25.5625rem; width: 8.5rem; font-size: 1.75rem;height: 4.25rem; background-color: gray; color: black; border: none; border-radius: 0.3125rem;" onclick={ctx.link().callback(|_| Msg::EnableNextRow)} disabled={self.all_submitted || self.inputs[self.current_row].iter().any(|s| s.is_empty())}>
                                { "ENTER" }
                        </button>
                    </div>
                </div>
                <div style="
                    position: fixed;
                    top: 12.5rem;
                    left: 1.875rem;
                    background: rgba(242, 237, 237, 0.8);  // 降低透明度
                    border: 0.0625rem solid #ccc;
                    padding: 0.625rem;
                    border-radius: 0.625rem;  // 圆角效果
                    box-shadow: 0 0.25rem 0.5rem rgba(0, 0, 0, 0.1);  // 添加阴影效果
                    z-index: 2000;
                    backdrop-filter: blur(0.3125rem);  // 添加模糊效果
                    -webkit-backdrop-filter: blur(0.3125rem);  // 添加模糊效果 (兼容WebKit)">
                    <p>{ format!("Total Rounds: {}", self.total_round) }</p>
                    <p>{ format!("Total Success: {}", self.total_success) }</p>
                    <p>{ format!("Average Round: {:.2}", self.average_round) }</p>
                    <ul>
                        { for self.words.iter().take(5).map(|(word, &count)| html! {
                            <li>{ format!("{}: {}", word, count) }</li>
                        })}
                    </ul>
                </div>
                <hr style="position: fixed; bottom: 1.875rem; left: 0; border: 0; border-top: 0.125rem solid #ccc; width: 100%; z-index: 999;" />
                <div style="
                position: fixed;
                bottom: 0.625rem;
                left: 50%;
                transform: translateX(-50%);
                text-align: center;
                font-size: 0.875rem;
                color: #333;
                z-index: 1000;">
                { "Powered by Rust & Yew & WebAssembly" }
            </div>
            <div style = "position:fixed; bottom : 25rem ; right : 0 ;transform: translateX(-50%); z-index: 500; ">
                <a href="https://github.com/Rosist-Sallina">
                        <img src="https://vip.helloimg.com/i/2024/07/11/668ec70f3019c.png" alt="zakozako" style="width: 21.875rem; height: auto;transform:rotate(5deg);" />
                </a>
            </div>
            </div>    
            </div>
        }
    }
    
}

impl Model {
    fn view_row(&self, ctx: &Context<Self>, row: usize) -> Html {
        let is_disabled = row > self.current_row || self.all_submitted;
        let is_clear_disabled = row != self.current_row || self.all_submitted;
        let flip_class = if self.flip_flags[row] { "flip-animation" } else { "" };
        html! {
            <div style="display: flex; gap: 5px; justify-content: center;" class={ flip_class }>
                { (0..5).map(|col| self.view_input(ctx, row, col, is_disabled)).collect::<Html>() }
                <button style="width: 68px; height: 68px; padding: 3px; font-size: 20px; text-align: center; border: 1px solid #ccc; border-radius: 3px;"
                        onclick={ctx.link().callback(move |_| Msg::ClearRow(row))} disabled={is_clear_disabled}>
                    { "Clear" }
                </button>
            </div>
        }
    }

    fn view_input(&self, ctx: &Context<Self>, row: usize, col: usize, is_disabled: bool) -> Html {
        let input_id = format!("input-{}-{}", row, col);
        let color = self.colors[row][col].clone();
        let text_color = if color == "green" || color == "#CCCC00" || color == "red" {
            "white"
        } else {
            "black"
        };
        let classname = "flip-rotate";
        html! {
            <>
                    <input
                        id={input_id}
                        type="text"
                        value={self.inputs[row][col].clone()}
                        oninput={ctx.link().callback(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let value = input.value().to_uppercase();  // 转换为大写
                            Msg::UpdateInput(row, col, value)
                        })}
                        disabled={is_disabled} 
                        class = {classname}
                        style={format!("width: 60px; height: 60px; padding: 3px; font-size: 28px; text-align: center; border: 1px solid #ccc; border-radius: 3px; background-color: {};color: {};", color, text_color)}
                        maxlength="1"
                    />
            </>
        }
    }
    fn view_keyboard(&self) -> Html {
        let keys_rows = vec![
            "QWERTYUIOP",
            "ASDFGHJKL",
            "ZXCVBNM",
        ];
        let key_colors = self.alphabet_color.chars().collect::<Vec<_>>();
        
        html! {
        <div style=" display: flex; position:fixed; bottom: 50px; flex-direction: column; align-items: center; gap: 5px; width: 100%; padding: 10px;">
            { for keys_rows.iter().enumerate().map(|(_row_index, row)| {
                html! {
                    <div style="display: flex; justify-content: center; gap: 5px;">
                        { for row.chars().enumerate().map(|(_col_index, key)| {
                            let color = match key_colors[key as usize - 'A' as usize] {
                                'G' => "green",
                                'Y' => "#CCCC00",
                                'R' => "red",
                                _ => "gray",
                            };
                            let text_color = if color == "green" || color == "#CCCC00" || color == "red" {
                                "white"
                            } else {
                                "black"
                            };
                            html! {
                                <div style={format!("width: 68px; height: 68px; font-size: 28px; display: flex; align-items: center; justify-content: center; background-color: {}; color: {}; border-radius: 5px;", color, text_color)}>
                                    { key }
                                </div>
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
    }


    fn validate_row(&self, row: usize) -> bool {
        let temp_str = self.inputs[row].iter().fold(String::new(), |acc, s| acc + s);
        let temp_str = temp_str.to_lowercase();
        if !select::ACCEPTABLE.contains(&temp_str.as_str()) || !self.inputs[row].iter().all(|s| s.chars().all(|c| c.is_ascii_alphabetic())) {
            return false;
        }
        else{
            true
        }
    }
    fn view_styles(&self) -> Html {
        html! {
            <style>
                {"
                    @keyframes flip {
                        0% {
                            transform: rotateX(0deg);
                        }
                        50% {
                            transform: rotateX(90deg);
                        }
                        100% {
                            transform: rotateX(180deg);
                        }
                    }
                    .flip-animation {
                        animation: flip 0.6s;
                    }
                "}
            </style>
        }
    }
}
