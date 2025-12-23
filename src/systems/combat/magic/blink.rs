use crate::systems::combat::CombatContext;

pub const fn perform_blink(ctx: &mut CombatContext) {
    ctx.transform.translation = ctx.cursor_pos.extend(0.0);
}
