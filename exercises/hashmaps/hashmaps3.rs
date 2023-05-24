// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        /*

        使用 HashMap::entry() 方法获取哈希表中的值时，该方法返回一个可变引用 (reference)，它是一个包含键值对的 Entry 枚举。这个可变引用有一个指向内部存储值的指针，我们需要使用该指针来访问存储在哈希表中的 Team 结构体的字段。

        因为 entry() 方法的返回值是一个指向被存储在哈希表的“值”，
        即存储在哈希表中的 Team 结构体的可变引用，因此我们需要对其进行解引用操作。
        这样可以在不复制整个 Team 结构的情况下访问它，从而提高程序的效率。当我们将 (*scope) 应用到指向可变引用的指针时，
        实际上是对指针指向的值进行了解引用操作，以获得对存储在哈希表中的 Team 结构体的字段的访问权限。
                 */
        let score = scores.entry(team_1_name.clone()).or_insert({
            Team {
                name: team_1_name,
                goals_scored: 0,
                goals_conceded: 0,
            }
        });
        (*score).goals_scored += team_1_score;
        (*score).goals_conceded += team_2_score;

        let score = scores.entry(team_2_name.clone()).or_insert(Team {
            name: team_2_name,
            goals_scored: 0,
            goals_conceded: 0,
        });
        (*score).goals_scored += team_2_score;
        (*score).goals_conceded += team_1_score;
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
