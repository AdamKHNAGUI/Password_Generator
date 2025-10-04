use rand::{distributions::Alphanumeric, Rng}; //lib pour générer des valeurs aléatoires
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // on vérifie si il y a un paramètre on le convertit en usize, valeur de 12 si pas de paramètre ou problème de conversion
    let length = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(12)
    } else {
        12
    };

    let password: String = rand::thread_rng()          //générateur aléatoire sécurisé pour le thread courant
        .sample_iter(&Alphanumeric) //crée un flux de caractères alphanumériques
        .take(length)               //garder le nombre de caractère demandé
        .map(char::from)           //convertit chaque bit en caractère
        .collect();                                    //récupère tous les caractères dans la variable String

    println!("Mot de passe généré: {}", password);
}