pub fn new_rate(winner_current_rate: u16, loser_current_rate: u16, k: u8) -> (u16, u16) {
    let loser_expected_win_rate = expected_win_rate(loser_current_rate, winner_current_rate);

    let winner_new_rate = (winner_current_rate as f64) + (k as f64) * loser_expected_win_rate;
    let loser_new_rate = (loser_current_rate as f64) - (k as f64) * loser_expected_win_rate;

    return (winner_new_rate as u16, loser_new_rate as u16);
}

fn expected_win_rate(winner_rate: u16, loser_rate: u16) -> f64 {
    return 1.0 / (10.0f64.powf((loser_rate as f64 - winner_rate as f64) / 400.0) + 1.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_win_rate_equal_ratings() {
        let rate_a = 1000;
        let rate_b = 1000;

        let expected_win_rate = expected_win_rate(rate_a, rate_b);

        assert_eq!(expected_win_rate, 0.5);
    }

    #[test]
    fn test_expected_win_rate_500_diff() {
        let rate_a = 1500;
        let rate_b = 1000;

        let rate = expected_win_rate(rate_a, rate_b);
        assert!(0.94 < rate && rate < 0.96);

        let rate_reverse = expected_win_rate(rate_b, rate_a);
        assert!(0.04 < rate_reverse && rate_reverse < 0.06);
    }

    #[test]
    fn test_new_rate_grather_and_less() {
        let winner_current_rate = 1000;
        let loser_current_rate = 1200;
        let k = 32;

        let (winner_new_rate, loser_new_rate) =
            new_rate(winner_current_rate, loser_current_rate, k);

        assert!(winner_new_rate > winner_current_rate); // 勝者はレートが上がっている
        assert!(loser_new_rate < loser_current_rate); // 敗者はレートが下がっている
    }

    #[test]
    fn test_new_rate_equal_ratings() {
        let winner_current_rate = 1500;
        let loser_current_rate = 1500;
        let k = 32;

        let (winner_new_rate, loser_new_rate) =
            new_rate(winner_current_rate, loser_current_rate, k);

        assert_eq!(winner_new_rate, 1516); // k * 勝率 0.5 が加算される
        assert_eq!(loser_new_rate, 1484); // k * 勝率 0.5 が減算される
    }

    #[test]
    fn test_new_rate_200_diff() {
        let winner_current_rate = 1500; // 勝率 0.75
        let loser_current_rate = 1300; // 勝率 0.25
        let k = 32;

        let (winner_new_rate, loser_new_rate) =
            new_rate(winner_current_rate, loser_current_rate, k);

        println!("winner_new_rate: {}", winner_new_rate);
        println!("loser_new_rate: {}", loser_new_rate);
        assert!(1507 <= winner_new_rate && winner_new_rate <= 1509); // k * 勝率 0.25 が加算される
        assert!(1291 <= loser_new_rate && loser_new_rate <= 1293); // k * 勝率 0.25 が減算される
    }

    #[test]
    fn test_new_rate_k_influence() {
        let winner_current_rate = 1100;
        let loser_current_rate = 1300;

        let (winner_new_rate_k32, loser_new_rate_k32) =
            new_rate(winner_current_rate, loser_current_rate, 32);
        let (winner_new_rate_k16, loser_new_rate_k16) =
            new_rate(winner_current_rate, loser_current_rate, 16);

        assert!(winner_new_rate_k32 > winner_new_rate_k16); // 勝者は K が大きいほど多く得る
        assert!(loser_new_rate_k32 < loser_new_rate_k16); // 敗者は K が大きいほど多く失う
    }
}
