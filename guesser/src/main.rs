use trie;

fn lookfor( t: &trie::Trie, word: &str ) {
    print!( "{}: ", word );
    if let Some( eowc ) = t.query( word ) {
        println!( "{}", eowc );
    } else {
        println!( "NotPresent" );
    }
}

fn main() {

    let mut t: trie::Trie = trie::Trie::new( );

    t.add( "adieu" );
    t.add( "rough" );
    t.add( "roast" );

    lookfor( &t, "roast" );
    lookfor( &t, "adieu" );
    lookfor( &t, "bitch" );

}
