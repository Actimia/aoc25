use std::{
  fmt::{Display, Write},
  str::FromStr,
};

use anyhow::{anyhow, bail, ensure};
use itertools::Itertools;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Suit {
  Spades,
  Hearts,
  Diamonds,
  Clubs,
}

impl TryFrom<char> for Suit {
  type Error = anyhow::Error;

  fn try_from(value: char) -> anyhow::Result<Self> {
    Ok(match value {
      '♠' | '♤' | 'S' | 's' => Suit::Spades,
      '♥' | '♡' | 'H' | 'h' => Suit::Hearts,
      '♦' | '♢' | 'D' | 'd' => Suit::Diamonds,
      '♣' | '♧' | 'C' | 'c' => Suit::Clubs,
      _ => bail!("unknown suit"),
    })
  }
}

impl Display for Suit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char(match *self {
      Suit::Spades => '♠',
      Suit::Hearts => '♥',
      Suit::Diamonds => '♦',
      Suit::Clubs => '♣',
    })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Rank {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl Rank {
  fn value(&self) -> u8 {
    match self {
      Rank::Two => 2,
      Rank::Three => 3,
      Rank::Four => 4,
      Rank::Five => 5,
      Rank::Six => 6,
      Rank::Seven => 7,
      Rank::Eight => 8,
      Rank::Nine => 9,
      Rank::Ten => 10,
      Rank::Jack => 11,
      Rank::Queen => 12,
      Rank::King => 13,
      Rank::Ace => 14,
    }
  }

  fn is_consecutive(&self, other: Rank) -> bool {
    match (self, other) {
      (Rank::Two, Rank::Ace) => true,
      (Rank::Ace, Rank::Two) => true,
      _ => self.value().abs_diff(other.value()) == 1,
    }
  }
}

impl TryFrom<char> for Rank {
  type Error = anyhow::Error;

  fn try_from(value: char) -> anyhow::Result<Self> {
    Ok(match value {
      '2' => Rank::Two,
      '3' => Rank::Three,
      '4' => Rank::Four,
      '5' => Rank::Five,
      '6' => Rank::Six,
      '7' => Rank::Seven,
      '8' => Rank::Eight,
      '9' => Rank::Nine,
      'T' | 't' => Rank::Ten,
      'J' | 'j' => Rank::Jack,
      'Q' | 'q' => Rank::Queen,
      'K' | 'k' => Rank::King,
      '1' | 'A' | 'a' => Rank::Ace,
      _ => bail!("unknown rank"),
    })
  }
}

impl Display for Rank {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char(match *self {
      Rank::Two => '2',
      Rank::Three => '3',
      Rank::Four => '4',
      Rank::Five => '5',
      Rank::Six => '6',
      Rank::Seven => '7',
      Rank::Eight => '8',
      Rank::Nine => '9',
      Rank::Ten => 'T',
      Rank::Jack => 'J',
      Rank::Queen => 'Q',
      Rank::King => 'K',
      Rank::Ace => 'A',
    })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub struct Card {
  rank: Rank,
  suit: Suit,
}

impl Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.rank, self.suit)
  }
}

impl TryFrom<&str> for Card {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> anyhow::Result<Self> {
    value.parse()
  }
}

impl FromStr for Card {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> anyhow::Result<Self> {
    let mut chars = s.chars();
    let rank = chars.next().ok_or(anyhow!("expected rank"))?;
    let suit = chars.next().ok_or(anyhow!("expected suit"))?;
    ensure!(chars.next().is_none(), "too long");

    Ok(Card {
      rank: rank.try_into()?,
      suit: suit.try_into()?,
    })
  }
}

impl Card {
  pub fn deck() -> Vec<Card> {
    static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
    static RANKS: [Rank; 13] = [
      Rank::Two,
      Rank::Three,
      Rank::Four,
      Rank::Five,
      Rank::Six,
      Rank::Seven,
      Rank::Eight,
      Rank::Nine,
      Rank::Ten,
      Rank::Jack,
      Rank::Queen,
      Rank::King,
      Rank::Ace,
    ];

    SUITS
      .iter()
      .cartesian_product(RANKS.iter())
      .map(|(suit, rank)| Card {
        suit: *suit,
        rank: *rank,
      })
      .collect()
  }
}

pub struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  pub fn from(mut cards: Vec<Card>) -> Self {
    assert_eq!(cards.len(), 5);
    cards.sort();
    Hand { cards }
  }

  pub fn evaluate(&self) -> Score {
    self
      .is_royal_flush()
      .or_else(|| self.is_straight_flush())
      .or_else(|| self.is_quads())
      .or_else(|| self.is_full_house())
      .or_else(|| self.is_flush())
      .or_else(|| self.is_straight())
      .or_else(|| self.is_three_of_a_kind())
      .or_else(|| self.is_two_pair())
      .or_else(|| self.is_pair())
      .unwrap_or_else(|| {
        let last = self.cards.last().unwrap().clone();
        Score::HighCard { high: last }
      })
  }

  pub fn is_pair(&self) -> Option<Score> {
    let (high, _) = find_with_count(&self.cards, 2)?;
    Some(Score::Pair { high })
  }

  pub fn is_two_pair(&self) -> Option<Score> {
    let (first, rest) = find_with_count(&self.cards, 2)?;
    let (second, _) = find_with_count(&rest, 2)?;
    Some(Score::TwoPair {
      high: first,
      low: second,
    })
  }

  pub fn is_three_of_a_kind(&self) -> Option<Score> {
    let (high, _) = find_with_count(&self.cards, 3)?;
    Some(Score::ThreeOfAKind { high })
  }

  pub fn is_full_house(&self) -> Option<Score> {
    let (three, rest) = find_with_count(&self.cards, 3)?;
    let (pair, _) = find_with_count(&rest, 2)?;
    Some(Score::FullHouse { three, pair })
  }

  pub fn is_quads(&self) -> Option<Score> {
    let (high, _) = find_with_count(&self.cards, 4)?;
    Some(Score::Quads { high })
  }

  pub fn is_straight(&self) -> Option<Score> {
    (self
      .cards
      .iter()
      .circular_tuple_windows()
      .filter(|(a, b)| a.rank.is_consecutive(b.rank))
      .count()
      == 4)
      .then(|| Score::Straight {
        high: self.cards.last().unwrap().clone(),
      })
  }

  pub fn is_flush(&self) -> Option<Score> {
    self
      .cards
      .iter()
      .map(|c| c.suit)
      .all_equal()
      .then(|| Score::Flush {
        high: self.cards.last().unwrap().clone(),
      })
  }

  pub fn is_straight_flush(&self) -> Option<Score> {
    self.is_straight()?;
    self.is_flush()?;
    Some(Score::StraightFlush {
      high: self.cards.last().unwrap().clone(),
    })
  }

  pub fn is_royal_flush(&self) -> Option<Score> {
    self.is_flush()?;
    if let Score::Straight { high } = self.is_straight()? {
      if high.rank == Rank::Ace {
        return Some(Score::RoyalFlush { high });
      }
    }
    None
  }
}

fn find_with_count(cards: &Vec<Card>, count: u8) -> Option<(Card, Vec<Card>)> {
  let counts = {
    let mut res = [0; 15];
    for card in cards {
      res[card.rank.value() as usize] += 1
    }
    res
  };

  if let Some((value, _)) = counts.iter().enumerate().rfind(|(_, c)| c >= &&count) {
    let (cards, rest): (Vec<Card>, Vec<Card>) = cards
      .iter()
      .cloned()
      .partition(|c| c.rank.value() == value as u8);
    return Some((cards.last().unwrap().clone(), rest));
  }

  None
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Score {
  HighCard { high: Card },
  Pair { high: Card },
  TwoPair { high: Card, low: Card },
  ThreeOfAKind { high: Card },
  Straight { high: Card },
  Flush { high: Card },
  FullHouse { three: Card, pair: Card },
  Quads { high: Card },
  StraightFlush { high: Card },
  RoyalFlush { high: Card },
}

#[cfg(test)]
mod tests {
  use super::*;

  fn hand(cards: [&str; 5]) -> Hand {
    Hand::from(cards.iter().map(|c| c.parse::<Card>().unwrap()).collect())
  }

  #[test]
  fn parse() {
    let card: Card = "A♥".parse().unwrap();
    assert_eq!(card.rank, Rank::Ace);
    assert_eq!(card.suit, Suit::Hearts);

    let card: Card = "2S".try_into().unwrap();
    assert_eq!(card.rank, Rank::Two);
    assert_eq!(card.suit, Suit::Spades);
  }

  #[test]
  fn display() {
    let card = Card {
      rank: Rank::Eight,
      suit: Suit::Clubs,
    };
    assert_eq!(format!("{}", card), "8♣");
  }

  #[test]
  fn test_deck() {
    let deck = Card::deck();
    assert_eq!(deck.len(), 52);
  }

  #[test]
  fn test_evaluate() {
    assert_eq!(
      hand(["5S", "AS", "7D", "8C", "9D"]).evaluate(),
      Score::HighCard {
        high: "AS".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "5D", "7D", "8C", "9D"]).evaluate(),
      Score::Pair {
        high: "5D".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "5D", "7D", "7C", "9D"]).evaluate(),
      Score::TwoPair {
        high: "7C".parse().unwrap(),
        low: "5D".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "5D", "5C", "7C", "9D"]).evaluate(),
      Score::ThreeOfAKind {
        high: "5C".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "6S", "7D", "8C", "9D"]).evaluate(),
      Score::Straight {
        high: "9D".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["QS", "KS", "AD", "2C", "3H"]).evaluate(),
      Score::Straight {
        high: "AD".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "JS", "QS", "KS", "AS"]).evaluate(),
      Score::Flush {
        high: "AS".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "5D", "5C", "7C", "7D"]).evaluate(),
      Score::FullHouse {
        three: "5C".parse().unwrap(),
        pair: "7C".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "5D", "5C", "5H", "9D"]).evaluate(),
      Score::Quads {
        high: "5C".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["5S", "6S", "7S", "8S", "9S"]).evaluate(),
      Score::StraightFlush {
        high: "9S".parse().unwrap()
      }
    );
    assert_eq!(
      hand(["TS", "JS", "QS", "KS", "AS"]).evaluate(),
      Score::RoyalFlush {
        high: "AS".parse().unwrap()
      }
    );
  }
}
