mod communication_system {
    use std::collections::HashSet;

    #[derive(Clone, Copy, Debug)]
    pub enum MarkerType {
        StartOfPacket = 4,
        StartOfMessage = 14,
    }

    #[derive(Debug)]
    pub struct Signal(Vec<char>);

    impl std::fmt::Display for Signal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for c in &self.0 {
                write!(f, "{}", c).unwrap();
            }

            Ok(())
        }
    }

    impl Signal {
        pub fn new(input: &Vec<String>) -> Result<Self, &str> {
            let bad_signal_msg = "Signal::new::Bad input";
            if input.len() > 1 {
                return Err(bad_signal_msg);
            }
            let signal: Vec<_> = input[0].chars().collect();
            if !signal.iter().all(|c| c.is_ascii_lowercase()) {
                return Err(bad_signal_msg);
            }

            Ok(Signal(signal))
        }

        pub fn get_start_of_packet_pos(&self, marker: MarkerType) -> Result<usize, &str> {
            let signal = &self.0;

            for i in 0..signal.len() {
                let marker_uniq_len = marker as usize;
                let uniq: HashSet<&char> = signal[i..i + marker_uniq_len].iter().collect();
                if uniq.len() == marker_uniq_len {
                    return Ok(i + marker_uniq_len);
                }
            }

            Err("Signal::get_start_of_packet_pos::Bad signal")
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize, usize> {
    let signal = communication_system::Signal::new(&input).unwrap();

    aoc::Answer(
        signal
            .get_start_of_packet_pos(communication_system::MarkerType::StartOfPacket)
            .unwrap(),
        signal
            .get_start_of_packet_pos(communication_system::MarkerType::StartOfMessage)
            .unwrap(),
    )
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(6, 7, 19).compute(&get_answer)
}
