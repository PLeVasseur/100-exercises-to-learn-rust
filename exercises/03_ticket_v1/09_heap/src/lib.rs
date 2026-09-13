pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), (usize::BITS / 8 * 3) as usize);
    }

    #[test]
    fn ticket_size() {
        assert_eq!(size_of::<Ticket>(), (usize::BITS / 8 * 3 * 3) as usize);
    }
}
