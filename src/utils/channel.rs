use std::{
    sync::mpsc::{Receiver, RecvError, RecvTimeoutError, SendError, Sender, TryRecvError, channel},
    time::Duration,
};

pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> (Self, Self) {
        let (sender_1, receiver_2) = channel();
        let (sender_2, receiver_1) = channel();
        (
            Self {
                sender: sender_1,
                receiver: receiver_1,
            },
            Self {
                sender: sender_2,
                receiver: receiver_2,
            },
        )
    }

    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        self.sender.send(message)
    }

    pub fn receive(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn receive_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    pub fn try_receive(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }
}

#[cfg(test)]
mod test {
    use std::{
        sync::mpsc::{RecvTimeoutError, TryRecvError},
        time::Duration,
    };

    use super::Channel;

    #[test]
    fn test_channel_send_receive() {
        let (left, right) = Channel::new();

        let handle = std::thread::spawn(move || {
            right.send(42).unwrap();
            assert_eq!(right.receive(), Ok(100));
        });

        assert_eq!(left.receive(), Ok(42));
        assert_eq!(left.send(100), Ok(()));

        handle.join().unwrap();
    }

    #[test]
    fn test_channel_timeout() {
        let (left, right) = Channel::new();

        assert_eq!(left.receive_timeout(Duration::from_millis(100)), Err(RecvTimeoutError::Timeout));

        assert_eq!(right.send(1), Ok(()));
        assert_eq!(left.receive_timeout(Duration::from_millis(100)), Ok(1));
    }

    #[test]
    fn test_channel_try_receive() {
        let (left, right) = Channel::new();

        assert_eq!(left.try_receive(), Err(TryRecvError::Empty));

        assert_eq!(right.send(1), Ok(()));
        assert_eq!(left.try_receive(), Ok(1));
    }

    #[test]
    fn test_channel_multiple_messages() {
        let (left, right) = Channel::new();

        std::thread::spawn(move || {
            for i in 0..5 {
                assert_eq!(right.send(i), Ok(()));
            }
        });

        for i in 0..5 {
            assert_eq!(left.receive().unwrap(), i);
        }
    }
}
