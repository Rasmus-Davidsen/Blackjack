use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use rand::random_range;
use crate::deck::{Deck, Card, Rank};

mod deck;

fn main() {
    start_game();
}

fn start_game() {
    let game = Game::new();
    prompt_bet(game);
}

fn prompt_bet(mut game : Game) {
    loop {
        println!();
        println!("Balance: €{}", game.player.balance);
        println!("Your bet, or blank to cash out: (Min €10)");

        let input = read_input();

        if input.trim().is_empty() {
            cash_out(game);
            return;
        }

        let result = input.trim().parse::<i32>();

        if result.is_err() {
            println!("\nPlease provide whole number");
            continue;
        }

        let bet = result.unwrap();

        if bet > game.player.balance {
            println!("\nYou don't have enough chips");
            continue
        }

        if bet < 10 {
            println!("\nYou can't bet less than €10");
            continue
        }

        game.bet = bet;
        break;
    }

    deal_cards(game);
}

fn cash_out(game : Game) {
    println!();
    wait(0.75);

    if game.player.balance < 10 {
        println!("You don't have enough to continue");
    }

    println!("You've cashed out with €{}!", game.player.balance);

    wait(1.5);

    println!();
    println!("Press enter to play again (Q = Quit)");

    let input = read_input().to_lowercase();

    if input.trim() == "q" || input.trim() == "quit" {
        exit(0);
    } else {
        start_game();
    }
}

fn deal_cards(mut game : Game) {
    deal_card(&mut game, false);
    deal_card(&mut game, true);

    deal_card(&mut game, false);
    deal_card(&mut game, true);

    prompt_turn(game);
}

fn deal_card(game : &mut Game, is_dealer : bool) {
    wait(0.75);

    if is_dealer {
        game.dealer.draw(&mut game.deck);
    } else {
        game.player.hand.draw(&mut game.deck);
    }

    render_game(game, true);
}

fn prompt_turn(mut game : Game) {
    render_game(&mut game, true);
    wait(1.0);

    let sum = game.player.hand.sum();

    if sum >= 21 {
        if sum > 21 {
            println!("\nBust! You went over 21");
        }

        wait(1.5);
        dealer_turn(game);
        return;
    }

    println!("[1 = Stand]  [2 = Hit]");

    match read_input().to_lowercase().trim() {
        "1" | "stand" =>  dealer_turn(game),
        "2" | "hit" => hit(game),
        _ => prompt_turn(game)
    }
}

fn hit(mut game : Game) {
    let hand = &mut game.player.hand;
    hand.draw(&mut game.deck);

    prompt_turn(game);
}

fn dealer_turn(mut game : Game) {
    render_game(&mut game, true);
    wait(1.0);

    render_game(&mut game, false);
    wait(1.0);

    while game.dealer.sum() < 17 {
        game.dealer.draw(&mut game.deck);
        render_game(&mut game, false);

        wait(0.75);
    }

    if game.dealer.sum() > 21 {
        println!("Dealer busted");
    }

    end_round(game);
}

fn end_round(mut game : Game) {
    let result = evaluate_round(&game);

    match result {
        Result::PlayerWins => {
            println!("You win! (+€{})", game.bet);
            game.player.balance += game.bet;
        }
        Result::DealerWins => {
            println!("Dealer wins (-€{})", game.bet);
            game.player.balance -= game.bet;
        }
        Result::Draw => {
            println!("It's a draw!");
        }
    }

    game.dealer.cards.clear();
    game.player.hand.cards.clear();

    wait(1.5);

    if game.player.balance < 10 {
        cash_out(game);
    } else {
        prompt_bet(game);
    }
}

fn evaluate_round(game : &Game) -> Result {
    let player_sum = game.player.hand.sum();
    let dealer_sum = game.dealer.sum();

    // Who busted
    if dealer_sum > 21 { // Dealer busted
        if player_sum > 21 { // Both busted
            return Result::Draw
        }

        // Dealer busted
        return Result::PlayerWins;
    } else if player_sum > 21 { // Player busted
        return Result::DealerWins
    }

    // Who has the greater sum
    if player_sum > dealer_sum {
        Result::PlayerWins
    } else if player_sum < dealer_sum {
        Result::DealerWins
    } else {
        Result::Draw
    }
}

fn render_game(game : &mut Game, hide_first : bool) {
    println!("\n");

    render_cards(String::from("Dealer:"), &game.dealer, hide_first);

    let label = format!(
        "Your hand: (Bet: €{})", game.bet
    );

    render_cards(label, &game.player.hand, false);
}

fn render_cards(label : String, hand : &Hand, hide_first : bool) {
    println!("{}", label);

    let mut is_first = true;
    let mut display = Vec::new();

    for card in &hand.cards {
        if is_first {
            is_first = false;

            if hide_first {
                display.push(String::from("🂠"));
                continue;
            }
        }

        display.push(card.name());
    }

    println!(" {}\n", display.join(" "));
}

fn read_input() -> String {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn wait(seconds : f32) {
    let ms = (seconds * 1000.0) as u32;
    let variation = (ms as f32 * 0.35) as u32;

    let min = ms - variation;
    let max = ms + variation;

    let delay = random_range(min..max);
    sleep(Duration::from_millis(delay as u64));
}

enum Result {
    PlayerWins, DealerWins, Draw
}

struct Player {
    balance: i32,
    hand: Hand
}

impl Player {
    fn new() -> Self {
        Self {
            balance: 250,
            hand: Hand::new()
        }
    }
}

struct Game {
    player : Player,
    dealer : Hand,
    deck : Deck,
    bet: i32
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player::new(),
            dealer: Hand::new(),
            deck: Deck::new(),
            bet: 0
        }
    }
}

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn new() -> Self {
        Self {
            cards: Vec::new()
        }
    }

    fn draw(&mut self, deck : &mut Deck) {
        self.cards.push(
            deck.draw()
        );
    }

    fn sum(&self) -> u8 {
        let mut sum = 0;
        let mut has_ace = false;

        for card in &self.cards {
            if card.rank == Rank::Ace && !has_ace {
                has_ace = true;
                continue;
            }

            sum += card.rank.value();
        }

        if has_ace {
            if sum + 11 > 21 {
                sum += 1;
            } else {
                sum += 11;
            }
        }

        sum
    }
}