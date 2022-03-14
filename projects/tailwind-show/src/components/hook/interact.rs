use super::*;

impl UseTailwind {
    pub fn MinifyToggle(&self) -> LazyNodes {
        let value = self.get_minify();
        let label = "Minify";
        let click = move |e: FormEvent| {
            let new = match e.value.parse::<bool>() {
                Ok(o) => o,
                Err(_) => return,
            };
            self.set_minify(new);
        };
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "{label}"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{value}",
                    oninput: click
                }
            }
        )
    }
    pub fn ObfuscateToggle(&self) -> LazyNodes {
        let value = self.get_obfuscate();
        let label = "Obfuscate";
        let click = move |e: FormEvent| {
            let new = match e.value.parse::<bool>() {
                Ok(o) => o,
                Err(_) => return,
            };
            self.set_obfuscate(new);
        };
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "{label}"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{value}",
                    oninput: click
                }
            }
        )
    }

    fn PreflightToggle(tw: &UseTailwind) -> LazyNodes {
        let v = tw.get_preflight();
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Preflight"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{v}",
                    oninput: move |e| tw.set_preflight(e)
                }
            }
        )
    }

    fn ModeSelect(tw: &UseTailwind) -> LazyNodes {
        let v = tw.get_mode();
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Compile Mode"
                }
                select {
                    class: "select select-primary w-full max-w-xs",
                    value: "{v}",
                    onchange: move |e| tw.set_mode(e),
                    option {
                        value: "m",
                        "Normal"
                    }
                    option {
                        value: "i",
                        "Inline"
                    }
                    option {
                        value: "s",
                        "Scoped"
                    }
                    option {
                        value: "k",
                        "DataKey"
                    }
                    option {
                        value: "v",
                        "DataValue"
                    }
                }
            }
        )
    }

    pub fn compile(&self, input: &str) -> (LazyNodes, LazyNodes) {
        let mut config = self.state.borrow_mut();
        config.clear();
        match config.compile_html(input) {
            Ok((a, b)) => (
                rsx! {
                    CodeRenderer {
                        code: a,
                        is_error: false,
                    }
                },
                rsx! {
                    CodeRenderer {
                        code: b,
                        is_error: false,
                    }
                },
            ),
            Err(e) => {
                let (a, b) = (e.to_string(), e.to_string());
                (
                    rsx! {
                        CodeRenderer {
                            code: a,
                            is_error: true,
                        }
                    },
                    rsx! {
                        CodeRenderer {
                            code: b,
                            is_error: true,
                        }
                    },
                )
            },
        }
    }
}
