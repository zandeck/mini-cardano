struct ClientHasAgency;
struct ServerHasAgency;
struct Terminated;

trait State {
    type Agency;
}
struct StIdle {}
struct StBusy {}
struct StDone {}

impl State for StIdle {
    type Agency = ClientHasAgency;
}
impl State for StBusy {
    type Agency = ServerHasAgency;
}
impl State for StDone {
    type Agency = Terminated;
}

struct MsgPing {}
struct MsgPong {}
struct MsgDone {}

trait Transition<FromState>
where
    FromState: State,
{
    type ToState: State;
    fn transition(&self, s: &FromState) -> Self::ToState;
}

impl Transition<StIdle> for MsgPing {
    type ToState = StBusy;
    fn transition(&self, s: &StIdle) -> Self::ToState {
        StBusy {}
    }
}

impl Transition<StBusy> for MsgPong {
    type ToState = StIdle;
    fn transition(&self, s: &StBusy) -> Self::ToState {
        StIdle {}
    }
}

impl Transition<StIdle> for MsgDone {
    type ToState = StDone;
    fn transition(&self, s: &StIdle) -> Self::ToState {
        StDone {}
    }
}

struct StateMachine<S: State> {
    state: S,
}

impl<S: State> StateMachine<S> {
    pub fn progress<M: Transition<S>>(&self, message: M) -> StateMachine<M::ToState> {
        StateMachine {
            state: message.transition(&self.state),
        }
    }
}

fn main() {
    let sm = StateMachine { state: StIdle {} };
    let sm1 = sm.progress(MsgPing {});

    // Doesn't work - type checking prevent invalid state transition
    let sm2 = sm1.progress(MsgPing {});

    // instead, this works
    let sm2 = sm1.progress(MsgPong {});
}
