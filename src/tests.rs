#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_balance() {
        let wallet = Wallet::new();
        assert_eq!(wallet.get_balance(), 0);
    }
    
    #[test]
    fn test_receive_tokens() {
        let mut wallet = Wallet::new();
        wallet.receive_tokens(50);
        assert_eq!(wallet.get_balance(), 50);
    }
    
    #[test]
    fn test_send_tokens() {
        let mut wallet1 = Wallet::new();
        let mut wallet2 = Wallet::new();
        wallet1.receive_tokens(100);
        assert!(wallet1.send_tokens(50, &mut wallet2).is_ok());
        assert_eq!(wallet1.get_balance(), 50);
        assert_eq!(wallet2.get_balance(), 50);
    }
    
    #[test]
    fn test_insufficient_balance() {
        let mut wallet1 = Wallet::new();
        let mut wallet2 = Wallet::new();
        wallet1.receive_tokens(20);
        assert!(wallet1.send_tokens(50, &mut wallet2).is_err());
    }
}
