use crate::color_util::ColorUtil;
use crate::state::Position;

use druid::{widget::*, *};

pub struct Tile<T: druid::Data, R: Widget<T>> {
    position: Position,
    inner: R,
    _t: std::marker::PhantomData<T>,
}

impl<T: druid::Data, R: Widget<T>> Tile<T, R> {
    pub fn new(position: Position, inner: R) -> Self {
        Self {
            position,
            inner,
            _t: std::marker::PhantomData::default(),
        }
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: druid::Data, R: Widget<T>> Widget<T> for Tile<T, R> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, &bc, data, env);
        bc.max()
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let pos = self.position;
        let checkerboard = pos.0 % 2 == pos.1 % 2;

        let bounds = ctx.size().to_rect();
        let is_active = ctx.is_active();
        let colo = ColorUtil::hsl(
            0.1,
            0.2,
            if checkerboard { 0.1 } else { 0.5 } + if is_active { 0.2 } else { 0. },
        );

        ctx.fill(bounds, &colo);
        self.inner.paint(ctx, data, env);
        // ctx.with_save(|ctx| {
        //     ctx.transform(Affine::translate(Vec2::from((8.0, 7.0))));
        //     self.label.paint(ctx, data, env);
        // });
    }
}
