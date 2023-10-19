use msstools::pw::{Pathway, Reaction};

#[test]
fn test_split() {
    let mut pathway = Pathway::new();
    let mut reaction1 = Reaction::new(0, "R0".to_string());

    reaction1.add_substrate(0);

    reaction1.add_product(1);
    reaction1.add_product(2);

    pathway.add_reaction(reaction1);

    assert_eq!(pathway.get_reactions_count(), 1);

    pathway.split_multiple_product();

    assert_eq!(pathway.get_reactions_count(), 2);
}

#[test]
fn test_duplicate() {
    let mut pathway = Pathway::new();
    let mut reaction1 = Reaction::new(0, "R0".to_string());
    let mut reaction2 = Reaction::new(1, "R1".to_string());

    reaction1.add_substrate(0);
    reaction1.add_product(1);

    reaction2.add_substrate(0);
    reaction2.add_product(1);

    pathway.add_reaction(reaction1);
    pathway.add_reaction(reaction2);

    assert_eq!(pathway.get_reactions_count(), 2);

    pathway.join_duplicates();

    assert_eq!(pathway.get_reactions_count(), 1);
}

#[test]
fn test_dominated() {
    let mut pathway = Pathway::new();
    let mut reaction1 = Reaction::new(0, "R0".to_string());
    let mut reaction2 = Reaction::new(1, "R1".to_string());

    reaction1.add_substrate(0);
    reaction1.add_product(1);

    reaction2.add_substrate(0);
    reaction2.add_product(1);
    reaction2.add_product(2);

    pathway.add_reaction(reaction1);
    pathway.add_reaction(reaction2);

    assert_eq!(pathway.get_reactions_count(), 2);

    pathway.join_dominated_product();

    assert_eq!(pathway.get_reactions_count(), 1);
}
