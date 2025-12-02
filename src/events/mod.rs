pub trait FromEvent
where
  Self: Sized,
{
  type Event;
  type Error;

  fn add_event(self, event: Self::Event) -> Result<Self, Self::Error>;

  fn add_events<I>(self, events: I) -> Result<Self, Self::Error>
  where
    I: IntoIterator<Item = Self::Event>,
  {
    events
      .into_iter()
      .try_fold(self, |res, event| res.add_event(event))
  }

  fn from_events<I>(events: I) -> Result<Self, Self::Error>
  where
    Self: Default,
    I: IntoIterator<Item = Self::Event>,
  {
    let res: Self = Default::default();
    res.add_events(events)
  }
}

/*
impl<T, I, E, Err> TryFrom<I> for T
where
T: Default + FromEvent<Event = E, Error = Err>,
I: IntoIterator<Item = E>,
{
  type Error = Err;

  fn try_from(events: I) -> Result<Self, Self::Error> {
    T::from_events(events)
  }
}
*/

#[cfg(test)]
mod test {
  use anyhow::ensure;

  use super::*;

  #[derive(Debug, Clone, PartialEq)]
  struct Account {
    id: usize,
    balance: usize,
  }

  impl Default for Account {
    fn default() -> Self {
      Self {
        id: 1234,
        balance: 0,
      }
    }
  }

  #[derive(Debug, Clone, PartialEq)]
  enum AccountEvent {
    Withdraw { amount: usize },
    Deposit { amount: usize },
  }

  impl FromEvent for Account {
    type Event = AccountEvent;
    type Error = anyhow::Error;

    fn add_event(mut self, event: Self::Event) -> Result<Self, Self::Error> {
      match event {
        AccountEvent::Withdraw { amount } => {
          ensure!(self.balance >= amount, "not enough funds",);
          self.balance -= amount;
        }
        AccountEvent::Deposit { amount } => {
          self.balance += amount;
        }
      }
      Ok(self)
    }
  }

  #[test]
  fn test_combine() {
    let initial = Account {
      id: 1234,
      balance: 0,
    };

    let evs = vec![
      AccountEvent::Deposit { amount: 100 },
      AccountEvent::Deposit { amount: 100 },
      AccountEvent::Withdraw { amount: 150 },
    ];

    let result = initial.add_events(evs);

    assert!(result.is_ok());
    let acc = result.unwrap();
    assert_eq!(acc.balance, 50);
  }

  #[test]
  fn test_combine_err() {
    let initial = Account {
      id: 1234,
      balance: 0,
    };

    let evs = vec![
      AccountEvent::Deposit { amount: 100 },
      AccountEvent::Withdraw { amount: 150 },
    ];

    let result = initial.add_events(evs);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not enough funds"));
  }

  #[test]
  fn test_from_events() {
    let evs = vec![
      AccountEvent::Deposit { amount: 200 },
      AccountEvent::Deposit { amount: 100 },
      AccountEvent::Withdraw { amount: 150 },
    ];

    let result = Account::from_events(evs);

    assert!(result.is_ok());
    let acc = result.unwrap();
    assert_eq!(acc.balance, 150);
  }
}
