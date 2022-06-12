pub enum PlayerStatus {
    Stopped,
    Paused(String, String, u8),
    Playing(String, String, u8),
}
