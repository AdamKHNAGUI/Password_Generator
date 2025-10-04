use rand::Rng; //lib pour générer des valeurs aléatoires
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // on vérifie si il y a un paramètre on le convertit en usize, valeur de 12 si pas de paramètre ou problème de conversion
    let length = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(12)
    } else {
        12
    };
    //tout les caractères utilisé pour la génération
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
0123456789!@#$%^&*()-_=+[]{};:,.<>?/|"
        .chars()
        .collect();

    let mut rng = rand::thread_rng();                                               //générateur aléatoire sécurisé pour le thread courant
    let password: String = (0..length)                                                     //crée un itérateur sur length
        .map(|_| charset[rng.gen_range(0..charset.len())])//récupère un caractère aléatoire dans charset,
        .collect();                                                                           //récupère tous les caractères dans la variable String

    println!("Mot de passe généré: {}", password);
}
