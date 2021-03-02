use std::io;
//use rand::Rng;
fn main() {
    /*
Pour convertir une température Fahrenheit en Celsius, 
il faut procéder comme suit : 
temperature_f = (temperature_c x 1.8) + 32 
A l’inverse, pour convertir une température Celsius en Fahrenheit, 
il faut appliquer la formule : 
temperature_c = (temperature_f – 32) / 1.8
*/   
    // ont vas demander a l'utilisateur quoi veut im convertir 
    loop {
        println!("------------------------------Choisisez Votre conversion --------------------------------");
    let mut _choix = String::new();
    println!(" 
    1-Pour convertir une température Fahrenheit en Celsius 
    2-pour convertir une température Celsius en Fahrenheit: {}",
     _choix );
    io::stdin()
                .read_line(&mut _choix)
                .expect("Échec de la lecture de l'entrée utilisateur");

    //onvertir en int 
    let _choix: i8 = match _choix.trim().parse() {
        Ok(nombre) => nombre,
        Err(_) => continue,

    };

     // on vas appeller les fonction en fonction du choix
     if _choix == 1 {
        _temperature_f();
        println!("Merci d'avoir utilisé le programme") ;
    } else if _choix == 2 {
        _temperature_c();
        println!("Merci d'avoir utilisé le programme") ;

    } else {
        println!("Le choix Saisie ne correspond à aucune conversion  Merci ") ;
    } ;

    break;
     
 }

    
 
 }

//temperature_f
fn _temperature_f(){
    loop {
        let mut temperature = String::new();
        println!("Saisir la température en Degré Celsuic   : {}",temperature);
        io::stdin()
                .read_line(&mut temperature)
                .expect("Échec de la lecture de l'entrée utilisateur");
    
        //coonversion en int
        let temperature: f32 = match temperature.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        let temperature_f = (temperature * 1.8) + 32.0;
        println!("Le resultat de cette conversion est  : {}",temperature_f);
        break;
    }
    
}
//temperature_c
fn _temperature_c(){
    loop {
        let mut temperature_f = String::new();
        println!("Saisir la température en Fahrenheit : {}",temperature_f);
        io::stdin()
                .read_line(&mut temperature_f)
                .expect("Échec de la lecture de l'entrée utilisateur");
    
        //conversion int
        let temperature_f: f32 = match temperature_f.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        let temperature_c = (temperature_f - 32.0) / 1.8;
        println!("Le resultat de cette conversion est  : {}",temperature_c);
        break;
    }

    
}