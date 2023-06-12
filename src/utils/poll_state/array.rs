use std::ops::{Deref, DerefMut};

use super::PollState;

pub(crate) struct PollArray<const N: usize> {
    state: [PollState; N],
}

impl<const N: usize> PollArray<N> {
    pub(crate) fn new() -> Self {
        Self {
            state: [PollState::default(); N],
        }
    }

    /// Mark all items as "completed"
    #[inline]
    pub(crate) fn set_all_completed(&mut self) {
        self.iter_mut().for_each(|state| {
            debug_assert!(
                state.is_ready(),
                "Future should have reached a `Ready` state"
            );
            state.set_consumed();
        })
    }

    /// Get an iterator of indexes of all items which are "ready".
    pub(crate) fn ready_indexes<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.iter()
            .cloned()
            .enumerate()
            .filter(|(_, state)| state.is_ready())
            .map(|(i, _)| i)
    }

    /// Get an iterator of indexes of all items which are "pending".
    pub(crate) fn pending_indexes<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.iter()
            .cloned()
            .enumerate()
            .filter(|(_, state)| state.is_pending())
            .map(|(i, _)| i)
    }
}

impl<const N: usize> Deref for PollArray<N> {
    type Target = [PollState];

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<const N: usize> DerefMut for PollArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
