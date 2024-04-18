use gpui::*;

pub struct Counter {
    count: i32,
}

impl Global for CounterModel {}

#[derive(Clone)]
pub struct CounterModel {
    pub inner: Model<Counter>,
}

impl CounterModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let this = Self {
            inner: cx.new_model(|_| Counter { count: 0 }),
        };
        cx.set_global(this.clone());
        this
    }
    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

actions!(counter, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(1000.), px(500.)).into(),
                }),
                ..Default::default()
            },
            |cx| {
                let view = RenderCounter::build(cx);
                view
            },
        );
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    })
}

pub struct RenderCounter {
    state: CounterModel,
}

impl RenderCounter {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let state = CounterModel::init(cx);
            RenderCounter { state }
        });
        view
    }
}

impl Render for RenderCounter {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering counter view");
        let counter_ref = self.state.inner.read(cx);

        let increment_button = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child("Increment")
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                std::dbg!("Incrementing counter");
                CounterModel::update(
                    |model, cx| {
                        model.inner.update(cx, |model, cx| {
                            model.count += 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
            });

        let decrement_button = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child("Decrement")
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                std::dbg!("Decrementing counter");
                CounterModel::update(
                    |model, cx| {
                        model.inner.update(cx, |model, cx| {
                            model.count -= 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
            });

        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(div().flex().flex_col().children(vec![
                            increment_button,
                            decrement_button,
                            div()
                                .bg(rgb(0x4caf50))
                                .text_color(rgb(0xffffff))
                                .child(
                                    format!("The number is: {}!", counter_ref.count.to_string())
                                ),
                        ]))
    }
}
