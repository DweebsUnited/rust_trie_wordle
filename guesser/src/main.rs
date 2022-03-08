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

    lookfor( &t, "roast" );
    lookfor( &t, "adieu" );
    lookfor( &t, "bitch" );

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
        }

        lookfor( &t, guess );

    }

    Ok(())

}
