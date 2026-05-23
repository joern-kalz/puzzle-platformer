use super::sprite::Sprite;

pub enum UpdateResult {
    Walking(Sprite),
    Falling(Sprite),
    Leaving(Sprite),
    Exploding(Sprite),
    Dead,
    Left,
}
