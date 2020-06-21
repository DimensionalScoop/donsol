use crate::card::Card;

struct GameBoard {
    room: Vec<Card>,
    draw_pile: Vec<Card>,

    can_escape: bool,

    shield_strength: i8,
    shield_last_hit_strength: i8,
    health: i8,
    xp: i8,
}
