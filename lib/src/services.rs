// Services module - Entry point for all service functionality
pub mod calculate_elos;

// Re-export the main Elo calculation function for easier access
pub use calculate_elos::update_elos_for_group;
