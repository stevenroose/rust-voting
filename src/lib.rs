extern crate num_rational;

pub mod highest_averages;

/// A trait for seat allocation algorithms.
pub trait AllocateSeats {
	/// Calculates the number of seats per party given a vector of the number
	/// of votes per party.
	fn allocate_seats(&self, nb_seats: usize, parties: Vec<usize>) -> Vec<usize>;
}
