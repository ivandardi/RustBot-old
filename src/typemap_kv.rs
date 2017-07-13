use typemap::Key;
use uptimer::Uptimer;

pub struct UptimerKey;

impl Key for UptimerKey {
    type Value = Uptimer;
}
