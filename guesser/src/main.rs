use trie;

use std::io::{ self, Write };

fn lookfor( t: &trie::Trie, word: &str ) {
    print!( "{}: ", word );
    if let Some( eowc ) = t.query( word ) {
        println!( "{}", eowc );
    } else {
        println!( "NotPresent" );
    }
}


fn main( ) -> Result<(), io::Error> {

    // let t: trie::Trie = trie::io::from_wordlist( "resources/words_alpha.txt" )?;
    // trie::io::write_text( &t, "resources/words_alpha.trie" );

    let t: trie::Trie = trie::io::read_text( "resources/wordle.trie" )?;

    // let t: trie::Trie = trie::io::from_wordlist_if( "resources/wordle.txt", | s: &str | s.len( ) == 5 )?;
    // trie::io::write_text( &t, "resources/wordle.trie" )?;

    // lookfor( &t, "roast" );
    // lookfor( &t, "adieu" );
    // lookfor( &t, "bitch" );


    let mut exclusion_list: String = String::new( );
    let mut inclusion_list: Vec<String> = vec![ String::new( ), String::new( ), String::new( ), String::new( ), String::new( ) ];
    let mut inclusion_set: String = String::new( );
    let mut fixed_list: String = "     ".to_string( );

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

        let pred = | s: &str, _used: u64, eowc: u64 | -> bool {

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
                let mut printing: bool = true;
                for l in inclusion_set.chars( ) {
                    if ! s.contains( l ) {
                        printing = false;
                    }
                }

                if printing {
                    println!( "{}", s );
                }

            }
            true

        };

        t.walk( pred );

    }

    Ok(())

}
