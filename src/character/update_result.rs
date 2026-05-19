use super::sprite::Sprite;

pub enum UpdateResult {
    Walking(Sprite),
    Falling(Sprite),
    Dead,
}
