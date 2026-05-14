use super::sprite::Sprite;

pub enum UpdateResult {
    Walking(Sprite),
    Dead,
}
