use crate::geom::Linear;
use crate::layout::{Expansion, Fixed, Stack};
use crate::prelude::*;

/// `box`: Layouts its contents into a box.
///
/// # Keyword arguments
/// - `width`: The width of the box (length or relative to parent's width).
/// - `height`: The height of the box (length or relative to parent's height).
pub fn boxed(mut args: Args, ctx: &mut EvalContext) -> Value {
    let snapshot = ctx.state.clone();

    let body = args.find::<SynTree>().unwrap_or_default();
    let width = args.get::<_, Linear>(ctx, "width");
    let height = args.get::<_, Linear>(ctx, "height");
    args.done(ctx);

    let dirs = ctx.state.dirs;
    let aligns = ctx.state.aligns;

    ctx.start_content_group();
    body.eval(ctx);
    let children = ctx.end_content_group();

    ctx.push(Fixed {
        width,
        height,
        child: LayoutNode::dynamic(Stack {
            dirs,
            children,
            aligns,
            expansion: Spec::new(
                Expansion::fill_if(width.is_some()),
                Expansion::fill_if(height.is_some()),
            )
            .switch(dirs),
        }),
    });

    ctx.state = snapshot;
    Value::None
}
