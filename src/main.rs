use std::env; //lib pour lire les arguments de la commande
use std::fs; // lib pour manipuler les fichiers

fn main() {
    // variable immuable de vecteur String qui récupère toute la commande et ses paramètres
   let args: Vec<String> = env::args().collect();

    //on vérifie si on a un argument pour le programme
    if args.len() < 2 {
        println!("Renseignez un fichier en paramètre : <file.txt>");
        return;
    } else if args.len() > 2 {
        println!("Renseignez un seul fichier en paramètre");
        return;
    }

    // on récupère le fichier en paramètre
    let filename = &args[1];

    // on lit le conetnue du fichier , renvoie un message d'erreur en cas de problème
    let content = fs::read_to_string(filename).expect("Erreur Lecture du fichier");

    // on récupère les mots en retirants les espaces, tabulations,etc...
    let words: Vec<&str> = content.split_whitespace().collect();

    // on récupère le nombre de lettre en comptant chauqe caractère du texte sans prendre en compte les espaces, tabulations, etc...
    let char = content.chars().filter(|c| c.is_alphabetic()).count();

    // on affiche le nombre de mots et de lettre dans le fichier
    println!("Nombre de mots : {}", words.len());
    println!("Nombre de lettre : {}", char);
}
