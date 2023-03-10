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

// I AM NOT DONE

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

//dheeraj: so this is how you show the type of &mut
fn update_team_scores(scores: &mut HashMap<String, Team>, team_name:&str, goals_scored:u8, goals_conceded:u8){
    match scores.contains_key(team_name){
        true => {
            let mut team = scores.get_mut(team_name).unwrap();
            team.goals_scored+=goals_scored;
            team.goals_conceded+=goals_conceded;
        },
        false => {
            scores.insert(team_name.to_string(), 
            Team {     
                name: team_name.to_string(), 
                goals_scored, 
                goals_conceded,
        });
    },
    }
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    // for r in results.lines() {
    //     let v: Vec<&str> = r.split(',').collect();
    //     let team_1_name = v[0].to_string();
    //     let team_1_score: u8 = v[2].parse().unwrap();
    //     let team_2_name = v[1].to_string();
    //     let team_2_score: u8 = v[3].parse().unwrap();
    //     // TODO: Populate the scores table with details extracted from the
    //     // current line. Keep in mind that goals scored by team_1
    //     // will be number of goals conceded from team_2, and similarly
    //     // goals scored by team_2 will be the number of goals conceded by
    //     // team_1.

    //     // update team 1 score
    //     update_team_scores(&mut scores, &team_1_name, team_1_score, team_2_score);
        
    //     // update team 2 score
    //     update_team_scores(&mut scores, &team_2_name, team_2_score, team_1_score);
        
    // }

    // //dheeraj BONUS TODO the above in a functional manner
    results.lines()
    .into_iter()
    //dheeraj:TIL flatmaps
    .flat_map(|result| {
        let result:Vec<&str> = result.split(",").collect();
        let (team1_score, team2_score) = (result[2].parse::<u8>().unwrap(), result[3].parse::<u8>().unwrap());
        [
            Team {
                name: result[0].to_string(),
                goals_scored: team1_score,
                goals_conceded: team2_score,
            },
            Team {
                name: result[1].to_string(),
                goals_scored: team2_score,
                goals_conceded: team1_score,
            }
        ]
    })
    // .for_each(|team| update_team_scores(&mut scores, &team.name, team.goals_scored, team.goals_conceded)); // one way to do it
    .for_each(|team_from_iter| {
        scores.entry(team_from_iter.name.to_string())
        .and_modify(|team| Team {
            name: team.name.to_string(),
            goals_scored: team.goals_scored+team_from_iter.goals_scored,
            goals_conceded: team.goals_conceded+team_from_iter.goals_conceded,
        })
        .or_insert(team_from_iter);
    }); // 2nd way

    //dheeraj: THINK which implementation would you prefer imperative vs functional. I'm unsure. I'm leaning towards imperative due to familiarity.

    //Wow! looks like this was supposed to be a simple task, and I blew up the loc
    // for r in results.lines() {
    //     let v: Vec<&str> = r.split(',').collect();
    //     let team_1_name = v[0].to_string();
    //     let team_1_score: u8 = v[2].parse().unwrap();
    //     let team_2_name = v[1].to_string();
    //     let team_2_score: u8 = v[3].parse().unwrap();
    //     // TODO: Populate the scores table with details extracted from the
    //     // current line. Keep in mind that goals scored by team_1
    //     // will be number of goals conceded from team_2, and similarly
    //     // goals scored by team_2 will be the number of goals conceded by
    //     // team_1.
        
    //     //dheeraj: TIL entry modify or insert syntax
    //     scores.entry(team_1_name)
    //     .and_modify(|team| Team {
    //         name: team.name,
    //         goals_scored: team.goals_scored+team_1_score,
    //         goals_conceded: team.goals_conceded+team_2_score,
    //     })
    //     .or_insert(Team {
    //         name: team_1_name,
    //         goals_scored: team_1_score,
    //         goals_conceded: team_2_score,
    //     });

    //     scores.entry(team_1_name)
    //     .and_modify(|team| Team {
    //         name: team.name,
    //         goals_scored: team.goals_scored+team_1_score,
    //         goals_conceded: team.goals_conceded+team_2_score,
    //     })
    //     .or_insert(Team {
    //         name: team_1_name,
    //         goals_scored: team_1_score,
    //         goals_conceded: team_2_score,
    //     });

    // }

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
