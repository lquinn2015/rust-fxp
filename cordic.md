
# Outline


# What is Cordic and what does it do

Cordic stands for Coordinate Rotation Digitial Computer, it is boardly speaking
an algorithm but also an idea for computer transcendental functions like 
Sine, Cosine, and Tangent. These functions could already be solved by using
taylor series as its commonly known that 

$$
\sinh(x) = \sum_{n=0}^{\inf} \frac{(-1)^n * x^{2n+1}}{(2n+1)!} = x - \frac{x^3}{3!} + \frac{x^5}{5!} + ..  \\
\cosh(x) = \sum_{n=0}^{\inf} \frac{(-1)^n * x^{2n}}{(2n)!} = x - \frac{x^3}{3!} + \frac{x^5}{5!} + .. 
$$

These work great but they are rather computationally expensive. The amount of
percision you need is challenging because intermediate values and successive
multiplications will cost accuracy. While you can solve this by using floating
point numbers these come with there own challenges and your system potentially
might not have floating point. But there is a bigger issue if you are using
floating point you are missing out on the fun that is fixed point integer
solution!

Instead Cordic offers a method that is hardware friendly that can implement even
when you only have shifters. The method is also surprisingly accurate! 

# How Cordic works
What is Cordic doing than? Well first lets set that stage say we want to
calculate sin(u) or cos(u). If this is the case we get the following diagram

                                                                                
                                                                                
       pi/2                                                                         
        |       .                                                               
        |      /|                                                               
        |     / |                                                               
        |  C /  |                                                               
        |   /   |                                                               
        |  /    |  B                                                            
        | /     |                                                               
        |/ u    |                                                               
        ------------->-----                                                     
        |   A        1                                                          
        |                                                                       
        |                                                                       
        |                                                                       
        |                                                                       
        |                                                                       
      -pi/2                                                                                                                                                    
        
Let C be a line of length 1 that terminates on the unit circle. Than we can say that
(A,B) = (cos(u), sin(u)). So the game is on than if we can generate C we have
both cosine and sine computed but the question becomes how? Well the solution is
we are going to rotate U(x,y) = [1, 0] by angle u which is known. This can be
write in matrix forum like 

$$
    \begin{bmatrix}
    A \\
    B 
    \end{bmatrix}  =
    \begin{bmatrix}
    cos(u) & - sin(u) \\
    sin(u) & cos(u) 
    \end{bmatrix}
    \begin{bmatrix}
    1 \\ 
    0
    \end{bmatrix}
$$


# What is fixed point
