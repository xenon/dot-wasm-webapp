use layout::{
    backends::svg::SVGWriter,
    gv::{DotParser, GraphBuilder},
};
use leptos::*;

const INITIAL_TEXT: &str = r#"digraph G {
    subgraph cluster_0 {
        a0 -> a1 -> a2 -> a3;
    }
    
    subgraph cluster_1 {
        node [style=filled];
        b0 -> b1 -> b2 -> b3;
    }
    start -> a0;
    start -> b0;
    a1 -> b3;
    b2 -> a3;
    a3 -> a0;
    a3 -> end;
    b3 -> end;
}"#;

// Returns Option<String> because if we don't pass an svg_writer it just returns a string
// This is useful for setting the initial value
fn rebuild_svg(dot: String, maybe_svg_writer: Option<&WriteSignal<String>>) -> Option<String> {
    let mut parser = DotParser::new(&dot);
    let result = parser.process();
    if let Ok(g) = result {
        let mut gb = GraphBuilder::new();
        gb.visit_graph(&g);
        let mut vg = gb.get();
        let mut svg = SVGWriter::new();
        vg.do_it(false, false, false, &mut svg);
        let mut svg = svg.finalize();
        for _ in 0.."<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>".len() {
            svg.remove(0);
        }
        if let Some(svg_writer) = maybe_svg_writer {
            svg_writer.set(svg);
            return None;
        } else {
            return Some(svg);
        }
    }
    return None;
}

#[component]
pub(crate) fn App() -> impl IntoView {
    let (svg, svg_writer) = create_signal(rebuild_svg(INITIAL_TEXT.to_string(), None).unwrap());
    view! {
        <div class="split-div">
            <div class="split-div-inner">
                <textarea
                    id="test"
                    inner_html=INITIAL_TEXT
                    on:input=move |ev| {
                        rebuild_svg(event_target_value(&ev), Some(&svg_writer));
                    }
                >
                </textarea>
            </div>
            <div class="split-div-inner"
                inner_html=move || { svg.get() }
                {untrack(move || rebuild_svg(INITIAL_TEXT.to_string(), &svg_writer)) }
            >
            </div>

        </div>
    }
}
