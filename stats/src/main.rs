use trie;

use std::fs::File;
//use std::path::Path;
use std::io::{ self, Write };

fn main( ) -> Result<(), io::Error> {

    let t: trie::Trie = trie::io::read_text( "resources/wordle.trie" )?;

    let mut stats = [[0u64; 27]; 5];
    let mut probabilities = [[0f64; 26]; 5];
    let ord_base = 'a' as u8;

    let mut func = | s: &str, _used: u64, eowc: u64 | -> bool {

        // Keep only 5 letter words
        if s.len( ) == 5 && eowc > 0 {

            for ( ldx, letter ) in s.chars( ).enumerate( ) {

                let ord = ( letter as u8 ) - ord_base;
                stats[ ldx ][ ord as usize ] += eowc;
                stats[ ldx ][      26      ] += eowc;

            }
            // Not actually needed for this wordlist :shrug:
            return false;

        }
        true

    };

    t.walk( &mut func );

    let f: File = File::create( "resources/wordle_stats.csv" )?;
    let mut w: io::BufWriter<File> = io::BufWriter::new( f );

    // Normalize
    for ldx in 0..5 {
       for ord in 0..26 {
           probabilities[ ldx ][ ord ] = stats[ ldx ][ ord ] as f64 / stats[ ldx ][ 26 ] as f64;
       }
    }

    for ldx in 0..5 {
        for ord in 0..25 {
            write!( w, "{},", probabilities[ ldx ][ ord ] )?;
        }
        write!( w, "{}\n", probabilities[ ldx ][ 25 ] )?;
    }

    Ok(())

}