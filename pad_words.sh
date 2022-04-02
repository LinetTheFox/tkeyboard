# This script is used to pad all the strings from words.txt to the specific length
# which is usually the length of longest word.
# AWK_COMMAND is used to be able to set specifier counter from the MAXLEN variable:
#
# Suppose, we want to set the length of all strings to 20, then we want to run
# { printf "%-20s\n", $0 } on each string - it basically says "Print the word as it
# is, and put spaces in the end of string, so that the resulting string is 20 chars
# (or, add (20 - word length) spaces in the end)"
# Since it's impossible (or too difficult) that number from env variable - that
# is MAXLEN here - I ended up setting the program for awk in a separate variable
# and only afterwards pass it to awk.
RESROOT=res

MAXLEN=$(awk '{ if ( length > L ) { L=length} }END{ print L}' ${RESROOT}/words.txt)
AWK_COMMAND="{ if ( length <= ${MAXLEN} ) { printf \"%-${MAXLEN}s\n\", \$0 } }"
awk "$AWK_COMMAND" ${RESROOT}/words.txt > ${RESROOT}/words_padded.txt
