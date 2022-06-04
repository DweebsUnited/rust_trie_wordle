#![feature(total_cmp)]

use trie;

use std::io::{ self, Write };

fn _lookfor( t: &trie::Trie, word: &str ) {
    print!( "{}: ", word );
    if let Some( eowc ) = t.query( word ) {
        println!( "{}", eowc );
    } else {
        println!( "NotPresent" );
    }
}

mod loading {
    use std::fs::File;
    use std::path::Path;
    use std::io::{ self, BufRead, Result };

    pub fn load_probabilities<P: AsRef<Path>>( path: P ) -> Result<[[f64; 26]; 5]> {

        let f: File = File::open( path )?;
        let lines: io::Lines<io::BufReader<File>> = io::BufReader::new( f ).lines( );

        let mut stats = [[0f64; 26]; 5];
        let mut ldx = 0;

        for line in lines {

            if let Ok( probabilities ) = line {

                let probabilities = probabilities.trim( ).split( ',' );

                for ( ord, probability ) in probabilities.enumerate( ) {

                    stats[ ldx ][ ord ] = probability.parse::<f64>( ).expect( "Bad float parse in this file!" );

                }

                ldx += 1;

            }
        }

        Ok( stats )

    }

    pub fn load_freqs<P: AsRef<Path>>( path: P ) -> Result<[f64; 26]> {

        let f: File = File::open( path )?;
        let lines: io::Lines<io::BufReader<File>> = io::BufReader::new( f ).lines( );

        let mut stats = [0f64; 26];
        let mut ldx = 0;

        for line in lines {

            if let Ok( freq ) = line {

                let freqs = freq.trim( );

                stats[ ldx ] = freq.parse::<f64>( ).expect( "Bad float parse in this file!" );

                ldx += 1;

            }
        }

        Ok( stats )

    }

}

fn calc_probability( s: &str, probabilities: &[[f64; 26]; 5], freqs: &[f64; 26] ) -> f64 {

    let mut p = 1f64;

    for ( ldx, l ) in s.chars( ).enumerate( ) {

        let ord = ( l as u8 ) - ( 'a' as u8 );
        let mut duplicate_prob = 1f64;

        if ldx > 0 {
            if s[ 0..ldx ].contains( l ) {
                duplicate_prob *= 0.5;
            }
        }

        p *= probabilities[ ldx ][ ord as usize ] * freqs[ ord as usize ] * duplicate_prob;

    }

    p

}


fn main( ) -> Result<(), io::Error> {

    let t: trie::Trie = trie::io::read_text( "resources/wordle.trie" )?;

    let mut exclusion_list: String = String::new( );
    let mut inclusion_list: Vec<String> = vec![ String::new( ), String::new( ), String::new( ), String::new( ), String::new( ) ];
    let mut inclusion_set: String = String::new( );
    let mut fixed_list: String = "     ".to_string( );
    let mut guessed_list: String = String::new( );

    let letter_probabilities: [[f64; 26]; 5] = loading::load_probabilities( "resources/wordle_stats.csv" )?;
    let letter_freq: [f64; 26] = loading::load_freqs( "resources/letterfreq.csv" )?;

    loop {

        print!( "> " );

        io::stdout( ).flush( )?;

        let mut guess = String::new( );

        io::stdin( )
            .read_line( &mut guess )
            .expect( "Failed to read line" );

        let guess: &str = guess.trim( );

        if guess == "exit" {
            break;
        } else if guess == "help" {

            println!( "Exclusion list: {}", exclusion_list );
            println!( "Inclusion set:  {}", inclusion_set );
            println!( "Fixed letters:  {}", fixed_list );

            continue;

        }

        print!( "# " );

        io::stdout( ).flush( )?;

        let mut pattern = String::new( );

        io::stdin( )
            .read_line( &mut pattern )
            .expect( "Failed to read line" );

        let pattern: &str = pattern.trim( );


        // Add letters to exclusion and inclusion lists
        for ( ldx, ( l, p ) ) in guess.chars( ).zip( pattern.chars( ) ).enumerate( ) {

            // Keep track of all letters guessed - Don't worry about duplicates for now
            guessed_list.push( l );

            if p == 'r' {

                // Grey - Letters not present in word
                exclusion_list.push( l );

            } else if p == 'y' {

                // Yellow - Letters in wrong position
                inclusion_list[ ldx ].push( l );
                if ! inclusion_set.contains( l ) {
                    inclusion_set.push( l );
                }

            } else if p == 'g' {

                // Green - Letters locked in
                fixed_list.remove( ldx );
                fixed_list.insert( ldx, l );

            }

        }

        let mut guesses_list: Vec<(String, f64)> = Vec::new( );
        let mut elim_guesses_list: Vec<(String, f64)> = Vec::new( );

        let mut guess_func = | s: &str, _used: u64, eowc: u64 | -> bool {

            // Early - skip words too long
            // We will check each letter as it passes
            if s.len( ) > 5 {
                return false;
            }

            let l_c: char = s.chars( ).last( ).unwrap( );
            let l_dx: usize = s.len( ) - 1;

            // If we have fixed letters, make sure it matches
            if fixed_list.chars( ).nth( l_dx ).unwrap( ) != ' ' {
                if l_c != fixed_list.chars( ).nth( l_dx ).unwrap( ) {
                    return false;
                }
            }

            // Ignore words that use excluded letters
            if let Some( _ ) = exclusion_list.find( l_c ) {
                return false;
            }

            // Ignore words that use letters found in the wrong spot
            if let Some( _ ) = inclusion_list[ l_dx ].find( l_c ) {
                return false;
            }

            // Keep only 5 letter words
            if s.len( ) == 5 && eowc > 0 {

                // Make sure all letters in inclusion set are used
                // Fixed list already handled
                let mut printing: bool = true;
                for l in inclusion_set.chars( ) {
                    if ! s.contains( l ) {
                        printing = false;
                    }
                }

                if printing {

                    let word_probability: f64 = calc_probability( s, &letter_probabilities, &letter_freq );
                    let word: ( String, f64 ) = ( s.to_string( ), word_probability );

                    let idx: usize = guesses_list.binary_search_by( |( _w, p )| p.total_cmp( &word_probability ) ).unwrap_or_else( |x| x );
                    guesses_list.insert( idx, word );

                    while guesses_list.len( ) > 10 {
                        guesses_list.pop( );
                    }

                }

                return false;

            }

            true

        };

        let mut elim_guess_func = | s: &str, _used: u64, eowc: u64 | -> bool {

            // Early - skip words too long
            // We will check each letter as it passes
            if s.len( ) > 5 {
                return false;
            }

            let l_c: char = s.chars( ).last( ).unwrap( );

            // Ignore words that use already guessed letters
            if let Some( _ ) = guessed_list.find( l_c ) {
                return false;
            }

            // Keep only 5 letter words
            if s.len( ) == 5 && eowc > 0 {

                let word_probability: f64 = calc_probability( s, &letter_probabilities, &letter_freq );
                let word: ( String, f64 ) = ( s.to_string( ), word_probability );

                let idx: usize = elim_guesses_list.binary_search_by( |( _w, p )| p.total_cmp( &word_probability ) ).unwrap_or_else( |x| x );
                elim_guesses_list.insert( idx, word );

                while elim_guesses_list.len( ) > 10 {
                    elim_guesses_list.pop( );
                }

                return false;

            }

            true

        };

        t.walk( &mut guess_func );
        t.walk( &mut elim_guess_func );

        println!( "Best guesses:" );
        for ( w, p ) in guesses_list.iter( ).rev( ) {
            println!( "  {}: {}", w, p );
        }

        println!( "Best elimination guesses:" );
        for ( w, p ) in elim_guesses_list.iter( ).rev( ) {
            println!( "  {}: {}", w, p );
        }

    }

    Ok(())

}