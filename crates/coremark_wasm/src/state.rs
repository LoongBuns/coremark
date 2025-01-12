use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Start,
    S1,
    Int,
    Float,
    S2,
    ExponentSign,
    Exponent,
    Scientific,
    Invalid,
}

impl State {
    pub fn transition(input: &[u8]) -> (State, Vec<State>) {
        let mut state = State::Start;
        let mut state_path = vec![state];

        for &ch in input {
            state = match state {
                State::Start => match ch {
                    b'0'..=b'9' => State::Int,
                    b'+' | b'-' => State::S1,
                    b'.' => State::Float,
                    _ => State::Invalid,
                },
                State::S1 => match ch {
                    b'0'..=b'9' => State::Int,
                    b'.' => State::Float,
                    _ => State::Invalid,
                },
                State::Int => match ch {
                    b'0'..=b'9' => State::Int,
                    b'.' => State::Float,
                    b'e' | b'E' => State::S2,
                    _ => State::Invalid,
                },
                State::Float => match ch {
                    b'0'..=b'9' => State::Float,
                    b'e' | b'E' => State::S2,
                    _ => State::Invalid,
                },
                State::S2 => match ch {
                    b'+' | b'-' => State::ExponentSign,
                    b'0'..=b'9' => State::Exponent,
                    _ => State::Invalid,
                },
                State::ExponentSign => match ch {
                    b'0'..=b'9' => State::Exponent,
                    _ => State::Invalid,
                },
                State::Exponent => match ch {
                    b'0'..=b'9' => State::Scientific,
                    _ => State::Invalid,
                },
                State::Scientific => match ch {
                    b'0'..=b'9' => State::Scientific,
                    _ => State::Invalid,
                },
                State::Invalid => break,
            };
            state_path.push(state);
        }

        if !State::is_valid_final_state(state) && state != State::Invalid {
            state = State::Scientific;
            state_path.push(State::Scientific);
        }

        (state, state_path)
    }

    fn is_valid_final_state(state: State) -> bool {
        matches!(state, State::Int | State::Float | State::Scientific)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transition_int() {
        let input = b"1234";
        let (state, path) = State::transition(input);
        assert_eq!(state, State::Int);
        assert_eq!(
            path,
            vec![
                State::Start,
                State::Int,
                State::Int,
                State::Int,
                State::Int
            ]
        );
    }

    #[test]
    fn test_state_transition_float() {
        let input = b"12.34";
        let (state, path) = State::transition(input);
        assert_eq!(state, State::Float);
        assert_eq!(
            path,
            vec![
                State::Start,
                State::Int,
                State::Int,
                State::Float,
                State::Float,
                State::Float
            ]
        );
    }

    #[test]
    fn test_state_transition_scientific() {
        let input = b"1.23e10";
        let (state, path) = State::transition(input);
        assert_eq!(state, State::Scientific);
        assert_eq!(
            path,
            vec![
                State::Start,
                State::Int,
                State::Float,
                State::Float,
                State::Float,
                State::S2,
                State::Exponent,
                State::Scientific
            ]
        );
    }

    #[test]
    fn test_state_transition_invalid() {
        let input = b"abc";
        let (state, path) = State::transition(input);
        assert_eq!(state, State::Invalid);
        assert_eq!(
            path,
            vec![State::Start, State::Invalid]
        );
    }
}
