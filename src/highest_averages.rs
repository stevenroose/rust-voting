use num_rational::Rational;

use super::AllocateSeats;

/// The specific method used to specify the divisors.
#[derive(Clone)]
pub enum Method {
    DHondt,
    SainteLague,
    Imperiali,
    HuntingtonHill,
    Danish,
}

/// An implementation of an iterator that produces the divisors.
struct Divisors {
    method: Method,
    idx: isize,
}

impl Iterator for Divisors {
    type Item = Rational;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;
        let result: Rational = match self.method {
            Method::DHondt => Rational::new(i + 1, 1),
            Method::SainteLague => Rational::new(i * 2 + 1, 1),
            Method::Imperiali => Rational::new(i + 2, 2),
            Method::HuntingtonHill => Rational::new((i + 1) * (i + 2), 1) * -1,
            Method::Danish => Rational::new(self.idx * 3 + 1, 1),
        };
        self.idx += 1;
        Some(result)
    }
}

/// Implements the highest average method or divisor method for seat allocation.
/// For more info: https://en.wikipedia.org/wiki/Highest_averages_method
pub struct HighestAverages {
    method: Method,
}

impl HighestAverages {
    pub fn new(method: Method) -> HighestAverages {
        HighestAverages { method: method }
    }

    /// Produce an iterator over the divisors.
    fn divisors(&self) -> Divisors {
        Divisors {
            method: self.method.clone(),
            idx: 0,
        }
    }
}

impl AllocateSeats for HighestAverages {
    fn allocate_seats(&self, nb_seats: usize, parties: Vec<usize>) -> Vec<usize> {
        // Keep a sorted list of tuples (party_index, row, quotient).
        let mut matrix = Vec::new();
        for (row, divisor) in self.divisors().enumerate() {
            // Add the new row to the matrix.
            for (idx, party) in parties.iter().enumerate() {
                let quotient = Rational::new(1, *party as isize) * divisor;
                matrix.push((idx, row, quotient));
            }

            // If the number of quotients is lower than the number of seats,
            // we need another row.
            if matrix.len() < nb_seats {
                continue;
            }

            // Sort by quotient so that the top items are the seat allocations.
            matrix.sort_by(|e1, e2| e1.2.cmp(&e2.2));

            // If any allocated seat is from the last row, we need another row,
            // otherwise we are finished.
            if !matrix[0..nb_seats].iter().any(|s| s.1 == row) {
                break;
            }
        }

        let mut seats = vec![0; parties.len()];
        for seat in matrix[0..nb_seats].iter() {
            seats[seat.0] += 1;
        }
        seats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Rational;

    fn take_n_divisors(method: Method, n: usize) -> Vec<Rational> {
        Divisors {
            method: method,
            idx: 0,
        }.take(n)
        .collect()
    }

    fn make_rationals(vec: Vec<(isize, isize)>) -> Vec<Rational> {
        vec.into_iter().map(|e| Rational::new(e.0, e.1)).collect()
    }

    #[test]
    fn divisors() {
        // Testing the first 5 elements of all divisors.
        assert_eq!(
            make_rationals(vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)]),
            take_n_divisors(Method::DHondt, 5)
        );
        assert_eq!(
            make_rationals(vec![(1, 1), (3, 1), (5, 1), (7, 1), (9, 1)]),
            take_n_divisors(Method::SainteLague, 5)
        );
        assert_eq!(
            make_rationals(vec![(2, 2), (3, 2), (4, 2), (5, 2), (6, 2)]),
            take_n_divisors(Method::Imperiali, 5)
        );
        //TODO(stevenroose) Huntington-Hill
        assert_eq!(
            make_rationals(vec![(1, 1), (4, 1), (7, 1), (10, 1), (13, 1)]),
            take_n_divisors(Method::Danish, 5)
        );
    }

    #[test]
    fn example_verkiezingen2018() {
        let allocator = HighestAverages::new(Method::Imperiali);
        let seats = allocator.allocate_seats(13, vec![480, 310, 940, 270]);
        assert_eq!(vec![3, 1, 8, 1], seats);
    }
}
