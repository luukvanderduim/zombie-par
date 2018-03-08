// Zombies, the intern in the lab over the hanging bridge TEDed
// 2018 Luuk van der Duim

extern crate term_cursor as cursor;
use std::sync::{Arc, Mutex};

extern crate rayon;
use rayon::prelude::*;
//use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LabStaffMember {
    minutes: i32,
    job_title: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Duo {
    duo_left: Option<LabStaffMember>,
    duo_right: Option<LabStaffMember>,
}
impl Duo {
    fn slowest(&self) -> i32 {
        if self.duo_left.unwrap().minutes < self.duo_right.unwrap().minutes {
            return self.duo_right.unwrap().minutes;
        } else {
            return self.duo_left.unwrap().minutes;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BridgeCrossing {
    first_couple_hence: Duo,
    first_forth: Option<LabStaffMember>,
    second_couple_hence: Duo,
    second_forth: Option<LabStaffMember>,
    last_couple_hence: Duo,
    total_passage_duration: i32,
}

impl BridgeCrossing {
    fn gen_total_passage_duration(&mut self) {
        self.total_passage_duration =
            self.first_couple_hence.slowest() + self.first_forth.unwrap().minutes
                + self.second_couple_hence.slowest()
                + self.second_forth.unwrap().minutes + self.last_couple_hence.slowest();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct StateOfSides {
    toe: [Option<LabStaffMember>; 4],
    tve: [Option<LabStaffMember>; 4],
}
impl StateOfSides {
    fn heen(&mut self, l: &Option<LabStaffMember>) -> () {
        if self.tve.contains(&l) {
            println!(
                "{} heeft dubbelganger aan veilige brugzijde!",
                &l.unwrap().job_title
            );
        } else {
            let index = self.tve
                .into_iter()
                .position(|elem| elem.is_none())
                .unwrap();
            std::mem::replace(&mut self.tve[index], *l);
        }
        if (self.toe).contains(&l) {
            for elem in self.toe.iter_mut().find(|&&mut elem| elem == *l) {
                elem.take();
            }
        }
    }
    fn terug(&mut self, l: &Option<LabStaffMember>) -> () {
        if (&self.toe).contains(&l) {
            println!(
                "{} heeft dubbelganger aan onveilige brugzijde!",
                &l.unwrap().job_title
            );
        } else {
            let index = self.toe
                .into_iter()
                .position(|elem| elem.is_none())
                .unwrap();
            std::mem::replace(&mut self.toe[index], *l);
        }
        if (&self.tve).contains(&l) {
            for elem in self.tve.iter_mut().find(|&&mut elem| elem == *l) {
                elem.take();
            }
        }
    }
    fn not_so_safe_side(&self) -> [Option<LabStaffMember>; 4] {
        return self.toe;
    }
    fn safe_side(&self) -> [Option<LabStaffMember>; 4] {
        return self.tve;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct AdmBridgeCrossing {
    tel: i32,
    fastest: BridgeCrossing,
}
impl AdmBridgeCrossing {
    fn count_and_assign_fastest(&mut self, d: BridgeCrossing) {
        self.tel += 1;
        print!("{}{:3.}", cursor::Goto(20, 2), &self.tel);
        if *&self.tel == 108 {
            println!("{}", cursor::Goto(0, 12));
        }
        if d.total_passage_duration < self.fastest.total_passage_duration {
            self.fastest = d;

            print!("{}", cursor::Clear);
            println!("================================================================");
            println!("  Tried {:3.} out of      crossings.", &self.tel);
            println!("  This is (one of) the most efficient order(s): ");
            println!("================================================================");
            println!(
                " ➜ First duo to cross: the {} and the {}",
                &self.fastest.first_couple_hence.duo_left.unwrap().job_title,
                &self.fastest.first_couple_hence.duo_right.unwrap().job_title
            );
            println!(
                " ← The first to return the lantern: {}",
                &self.fastest.first_forth.unwrap().job_title
            );
            println!(
                " ➜ Second duo to cross are the {} and the {}",
                &self.fastest.second_couple_hence.duo_left.unwrap().job_title,
                &self.fastest
                    .second_couple_hence
                    .duo_right
                    .unwrap()
                    .job_title
            );
            println!(
                " ← The second to return the lantern: {}",
                &self.fastest.second_forth.unwrap().job_title
            );
            println!(
                " ➜ Last duo to cross are the {} amd the {}.",
                &self.fastest.last_couple_hence.duo_left.unwrap().job_title,
                &self.fastest.last_couple_hence.duo_right.unwrap().job_title
            );
            println!("================================================================");
            println!(
                "  These cross in {} minutes.",
                &self.fastest.total_passage_duration
            );
            println!("================================================================");
        }
    }
}

fn main() {
    let prof = LabStaffMember {
        minutes: 10,
        job_title: "Professor".as_ref(),
    };
    let tech = LabStaffMember {
        minutes: 5,
        job_title: "Labtechnician".as_ref(),
    };
    let jani = LabStaffMember {
        minutes: 2,
        job_title: "janitor".as_ref(),
    };
    let inte = LabStaffMember {
        minutes: 1,
        job_title: "intern".as_ref(),
    };
    let initduo = Duo {
        duo_left: None,
        duo_right: None,
    };

    let initstate = StateOfSides {
        tve: [None, None, None, None],
        toe: [Some(prof), Some(tech), Some(jani), Some(inte)],
    };

    let init_passage_seq = BridgeCrossing {
        first_couple_hence: initduo,
        first_forth: None,
        second_couple_hence: initduo,
        second_forth: None,
        last_couple_hence: initduo,
        total_passage_duration: 50,
    };

    let eff_cross_seq_solution = init_passage_seq;

    let admi = AdmBridgeCrossing {
        fastest: eff_cross_seq_solution,
        tel: 0,
    };

    let administratie = Arc::new(Mutex::new(admi));

    fn genereer_duos(knt: &[Option<LabStaffMember>]) -> Vec<Duo> {
        let mut tduo = Duo {
            duo_left: None,
            duo_right: None,
        };
        let mut duos_vec: Vec<Duo> = Vec::new();
        let makep = |k: usize| {
            for x in 0..k {
                if knt[x].is_none() {
                    continue;
                }
                for y in (x + 1)..k {
                    if knt[y].is_none() {
                        continue;
                    }
                    tduo.duo_left = knt[x];
                    tduo.duo_right = knt[y];
                    duos_vec.push(tduo);
                }
            }
            duos_vec
        };
        return makep(4);
    }

    let duos_vec = genereer_duos(&initstate.not_so_safe_side());

    duos_vec.into_par_iter().for_each(|v| {
        let mut state_of_sides = initstate;
        let mut current_crossing = init_passage_seq;
        current_crossing.first_couple_hence = v;

        state_of_sides.heen(&v.duo_left);
        state_of_sides.heen(&v.duo_right);

        for lantern in state_of_sides.safe_side().into_iter() {
            if lantern.is_none() {
                continue;
            }
            current_crossing.first_forth = *lantern;

            state_of_sides.terug(lantern);

            for w in genereer_duos(&state_of_sides.not_so_safe_side()) {
                current_crossing.second_couple_hence = w;

                state_of_sides.heen(&w.duo_left);
                state_of_sides.heen(&w.duo_right);

                for lantern in state_of_sides.safe_side().into_iter() {
                    if lantern.is_none() {
                        continue;
                    }
                    current_crossing.second_forth = *lantern;
                    state_of_sides.terug(lantern);

                    for u in genereer_duos(&state_of_sides.not_so_safe_side()) {
                        current_crossing.last_couple_hence = u;
                        current_crossing.gen_total_passage_duration();

                        // At this point current_crossing is allowed to be consumed
                        (*administratie)
                            .lock()
                            .unwrap()
                            .count_and_assign_fastest(current_crossing);
                    }
                    state_of_sides.heen(lantern);
                }
                state_of_sides.terug(&w.duo_left);
                state_of_sides.terug(&w.duo_right);
            }
            state_of_sides.heen(lantern);
        }
        // No more state dependent calls in this loop.
        // therefor there is no need to return the last two,
        // to recreate the initial state of affairs.
    });
} // End of main