fn main() {
    let input = include_str!("../input.txt");
    let mut sum = 0;
    for game in input.lines() {
        let game = Game::parse(game).unwrap();
        if game.draws().all(|d| d.is_valid()) {
            sum += game.id;
        }
    }
    println!("sum: {sum}");
}

struct Game<'a> {
    id: usize,
    draws: &'a str,
}

impl Game<'_> {
    fn parse(game: &str) -> Option<Game> {
        let game = game.strip_prefix("Game ")?;
        let mut split = game.split(':');
        let id = split.next()?.parse::<usize>().ok()?;
        let draws = split.next()?;
        Some(Game { id, draws })
    }

    fn draws(&self) -> impl Iterator<Item = Draw> + '_ {
        self.draws.split("; ").flat_map(|draw| {
            let items = draw.split(", ");
            items.flat_map(|item| {
                let mut item = item.trim().split(' ');
                let count = item.next()?.parse::<u8>().ok()?;
                let color = match item.next()? {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => return None,
                };
                Some(Draw { color, count })
            })
        })
    }
}

struct Draw {
    color: Color,
    count: u8,
}

enum Color {
    Red,
    Green,
    Blue,
}

impl Draw {
    pub fn is_valid(&self) -> bool {
        match self.color {
            Color::Red => self.count <= 12,
            Color::Green => self.count <= 13,
            Color::Blue => self.count <= 14,
        }
    }
}
