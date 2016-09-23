area_needed = 0
ribbon = 0
with open('input.txt', 'r') as file:
    for line in file:
        data = line.split('x')
        l = int(data[0])
        w = int(data[1])
        h = int(data[2])
        area_needed = 2*l*w + 2*w*h + 2*h*l + min(l*w,w*h,h*l) + area_needed
        
        ribbon = sorted([l,w,h])[1]*2+sorted([l,w,h])[0]*2 + ribbon
        ribbon = l*w*h + ribbon
print('Area of wrapping paper needed:')      
print(area_needed)
print('Ribbon Area')
print(ribbon)
input('Press ENTER to exit: ' )


