f = open('input.txt','r+')
print('Hello World')

floor = 0 
up = '('
down = ')'
firsttime_basement = 0
y = len(f.read())
for x in range (0, y):
   f.seek(x)
   character = f.read(1)
   if character == up:
      floor = floor+1
   elif character == down: 
      floor = floor-1
   else:
      print('ERROR')
   if floor == -1 and firsttime_basement == 0:
      firsttime_basement = x + 1
      print('Hey, its'+ str(floor))
print('Number of Floors: ')
print(floor)
print('First time at -1: ')
print(firsttime_basement)
input('Press ENTER to exit: ' )