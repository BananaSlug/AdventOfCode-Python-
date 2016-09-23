area_needed = 0
ribbon = 0
alphabet = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"
with open ('output.txt', 'w') as output:
   for i in range(27):
      output.write('\n----------------------------- %d -----------------------------\n' % i) 
      with open('input.txt', 'r') as file:
         for word in file:
            for ch in word:
               if ord(ch.lower()) < 96:
                  output.write(ch)
               else:
                  output.write(alphabet[ord(ch.lower())-97 + i])
      output.write("\n\n")      
         

input('Press ENTER to exit: ' )


