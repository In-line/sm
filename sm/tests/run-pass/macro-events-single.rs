extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked }

        TurnKey {
            Locked => Locked
        }
    }
}

fn main() {}
