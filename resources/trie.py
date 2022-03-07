import bisect
import struct

# Don't need to do this with a class, but its easier :shrug:
class Trie:
	def __init__( self, c, _used=0, _eowc=0 ):
		# Label
		self.c = c
		assert( len( self.c ) == 1 )

		# Stats -> number of times used, number of times it was the last character
		self.used = _used
		self.eowc = _eowc

		# Children, and precomputed keys
		self.children = [ ]
		self.children_keys = [ ]


	# Query if a child with given label exists
	def getChild( self, c ):
		child_dx = bisect.bisect_left( self.children_keys, c )

		if child_dx != len( self.children ) and self.children[ child_dx ].c == c:
			return self.children[ child_dx ]

		return None

	# Helper -> Add a Trie node as a child
	def _addChild( self, t ):
		child_dx = bisect.bisect_left( self.children_keys, t.c )

		self.children_keys.insert( child_dx, t.c )
		self.children.insert( child_dx, t )

		return child_dx

	# Add a new blank child for a given label
	def addChild( self, c ):
		c_node = Trie( c )
		self._addChild( c_node, c )

		return c_node

	# Query a child with given label, adding it if it does not exist
	# Duplication of child index query, but I'd rather do it this way
	def getOrAddChild( self, c ):
		child_dx = bisect.bisect_left( self.children_keys, c )

		if child_dx != len( self.children ) and self.children[ child_dx ].c == c:
			return self.children[ child_dx ]

		# Can insert on any dx returned by bisect
		c_node = Trie( c )

		self.children_keys.insert( child_dx, c )
		self.children.insert( child_dx, c_node )

		return c_node

	# TODO: Merge two nodes ( merge upper and lowercase for example, or all punctuation )
	# Sum used, eowc, mergesort-ish children


	# Add new word skipping prefix
	def accept( self, s, s_st=0 ):
		self.used += 1

		# Last char, no recurse
		# Return number of times this word has been accepted
		if s_st == len( s ):
			self.eowc += 1
			return self.eowc

		# Need a child if it doesnt exist
		else:
			# Get the char, create a kid if need be, recurse
			c_c = s[ s_st ]

			child = self.getOrAddChild( c_c )
			return child.accept( s, s_st + 1 )

	# Get number of times we have seen a word, skipping prefix
	def registered( self, s, s_st=0 ):
		# Last char -> Return number of times we've seen the word ( might be 0 )
		if s_st == len( s ):
			return self.eowc

		# Get next letter
		c_c = s[ s_st ]
		c = self.getChild( c_c )

		# No child -> Return how long existing prefix is, but avoid -0
		if c is None:
			return -( s_st + 1 )

		# Child -> Recurse
		else:
			return c._registered( s, s_st + 1 )


	# Serializing!
	def __str__( self ):
		return "{}:{}:{}:{}".format( self.c, self.used, self.eowc, len( self.children ) )

	# Consistency validation
	def validate( self, startFromRoot=True ):
		# root.eowc = 0
		if startFromRoot and self.eowc != 0:
			return False

		# used >= eowc -> Can't have ended more times than you were used
		if self.used < self.eowc:
			return False

		# used - eowc = sum( child.used ) -> Non-ending usage must match child total usage
		# all( child.validate ) -> Convenient time to recurse as well...
		cusedsum = 0
		for c in self.children:
			cusedsum += c.used
			if not c.validate( startFromRoot = False ):
				return False

		if ( self.used - self.eowc ) != cusedsum:
			return False

		return True


	# Now comes the fun! Statistics!
	# Lets get stupid with it...
	# First, we define a pre-order-traversal pattern that takes a function to call on entering and exiting each node
	# Everything else can be built from this boilerplate ( at the expense of a few more stack frames :shrug: )
	def nop( *args ):
		pass
	def _walk( self, nodeEntry, nodeExit ):
		nodeEntry( self )
		for c in self.children:
			c._walk( nodeEntry, nodeExit )
		nodeExit( self )

	# Walking starting at root should skip it
	# We want to be able to build all paths without having to specially handle the root node
	def walk( self, nodeEntry, nodeExit=nop ):
		for c in self.children:
			c._walk( nodeEntry, nodeExit )



# Export to file
def trieExport( t, fname ):
	# Buffer for filling - "cBQQ" -> 1,1,8,8 = 18 bytes per node
	buffer = bytearray( 18 )

	with open( fname, "wb" ) as outfile:

		# Export one - closure ;)
		def exportNode( n ):
			# Write self
			# For child -> recurse
			outfile.write( str( n ) + "\n" )

			for c in n.children:
				exportNode( c )

		# Kicker!
		exportNode( t )

# Import from file
def trieImport( fname ):
	# Buffer for parsing - "cBQQ" -> 1,1,8,8 = 18 bytes per node
	buffer = bytearray( 18 )

	with open( fname, "rb" ) as infile:

		def importNode( ):
			line = infile.readline( ).strip( )
			# print( "Building node -> {}".format( line ) )
			c, line = line[ 0 ], line[ 2: ]
			used, eowc, nc = line.split( ':' )
			# print( "{} -> {}:{}:{}".format( c, used, eowc, nc ) )

			node = Trie( c, _used = int( used ), _eowc = int( eowc ) )
			for cdx in range( int( nc ) ):
				# print( "Adding child {} / {}".format( cdx, int( nc ) ) )
				node._addChild( importNode( ) )

			return node

		return importNode( )

# Convert wordlist to new Trie
def trieWordlist( fname ):
	print( "Loading wordlist: {}".format( fname ) )
	root = Trie( '\0' )
	print( "Loading", end='' )
	with open( fname, "r" ) as infile:
		for word in infile.readlines( ):
			word = word.strip( )
			if len( word ) > 0:
				#print( word + ": " + str( root.accept( word ) ) )
				root.accept( word )

				if root.used % 1000 == 0:
					print( ".", end='' )

	print( "Done." )
	print( "Loaded {} words".format( root.used ) )
	return root


# Statistics functions!
# Find longest word
def trieStatsLongestWord( t ):
	longest = ""
	pathstack = [ ]

	def nodeEntry( n ):
		nonlocal longest

		pathstack.append( n.c )

		if len( pathstack ) > len( longest ):
			longest = ''.join( pathstack )

	def nodeExit( n ):
		pathstack.pop( )

	t.walk( nodeEntry, nodeExit )

	return longest

# Put together an alphabet of all characters
def trieStatsAlphabet( t ):
	a = set( )

	def nodeEntry( n ):
		a.add( n.c )

	t.walk( nodeEntry )

	return a


# Letter <-> word length <-> positional ocurrence counts
# This makes a fuck-off big matrix :D
def trieStatsPosFreq( t ):
	import numpy as np

	alpha = list( trieStatsAlphabet( t ) )
	alpha.sort( )
	longest = len( trieStatsLongestWord( t ) )

	print( "Creating PositionOcurrence array of size: {}x{}x{}".format( len( alpha ), longest, longest ) )
	# Array: a[ Letter ][ WordLength ][ PositionOcurrence ]
	a = np.zeros( ( len( alpha ), longest, longest ) )
	print( a.shape )

	pathstack = [ ]

	def nodeEntry( n ):
		pathstack.append( bisect.bisect_left( alpha, n.c ) )

		# Short circuit if no word ends here
		if n.eowc == 0:
			return

		for ldx, l_alpha_dx in enumerate( pathstack ):
			# print( "{}:{}:{} += {}".format( l_alpha_dx, len( pathstack ), ldx, n.eowc ) )
			a[ l_alpha_dx ][ len( pathstack ) - 1 ][ ldx ] += n.eowc

	def nodeExit( n ):
		pathstack.pop( )

	t.walk( nodeEntry, nodeExit )

	return ( a, alpha, longest )

	# Recommended normalization along one axis

	# l_sums = a.sum( axis = 0 )
	# I hate everything about this :D
	# for ldx in range( len( alpha ) ):
		# for wldx in range( longest ):
			# for podx in range( longest ):
				# Skip 0 entries
				# if l_sums[ wldx ][ podx ] > 0.5:
					# a[ ldx ][ wldx ][ podx ] /= l_sums[ wldx ][ podx ]

# TODO: Longest prefix(es) -> Longest prefix without ending a word

# TODO: Most common prefix? -> Prefix with highest used while eowc is 0

# TODO: Word length stats

# TODO: Alphabet diversity ( # children ) at each position

# TODO: Frequency of first and last letters, regardless of length

if __name__=="__main__":
	import sys
	root = trieImport( sys.argv[ 1 ] )
	print( root.validate( ) )
	print( "Loaded trie containing {} words".format( root.used ) )