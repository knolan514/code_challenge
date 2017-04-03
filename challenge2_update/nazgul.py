# import text
import sys
# search for patterns in text
import re
# if the command line only receives one argument (i.e $python nazgul.py) it will send an error message because you need to send the text file as an argument as well
if len(sys.argv) != 2:
    print "Error: Enter 1 argument."
else:
    with open(sys.argv[1], 'r') as textFile:
        # opens file
        text = textFile.read()
        # removes whitespace, and characters from the string then splits each phrase into its own line
        text = re.sub("[^\w\n]","",text).split('\n')
# a for loop that goes through textFile and determines if each string is a palindrome
# phrase is a new variable that takes one of the srings from the list at a time
for phrase in text:
    # makes all the letters lowercase
    phrase = phrase.lower()
    # rev_phrase is a new variable that is the reverse order of phrase
    rev_phrase = phrase[::-1]
    # if statement to see if phrase is equal to rev_phrase
    if phrase == rev_phrase:
        # sorts letters in rev_phrase reverse alphabetical
        rev_phrase = sorted(rev_phrase, reverse=True)
        # gets rid of the characters and whitspace
        rev_phrase = ''.join([char for char in rev_phrase if char.isalpha()])
        # if phrase is equal to rev_phrase then is prints AY + rev_phrase
        print 'AY |', rev_phrase
    else:
        rev_phrase = sorted(rev_phrase, reverse=True)
        rev_phrase = ''.join([char for char in rev_phrase if char.isalpha()])
        # if they are not equal then it prints NAY + rev_phrase
        print 'NAY |', rev_phrase
