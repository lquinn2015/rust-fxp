
# Forward 

As I redid this solution in verilog, I decided to change my rust implementation. Sadly
already wrote my rust solution out. My implmentation in rust will not much the soruce code.
Instead of trying to rotate v  = (1,0,0) to (sin($\theta$, cos($\theta$), $theta$). 
I choose to rotate v = (1, 0, $\theta$) to (sin($\theta$, cos($\theta$), 0).
This makes the $\theta$ compare easier as you can just check if is positive or
negative. It also allows you not to store theta which saves a register in
verilog. You just have to be careful with how you find the next $\theta_i$.
This was much easier to implement in verilog correctly!


# What is Cordic and what does it do

Cordic stands for Coordinate Rotation Digitial Computer, it is boardly speaking
an algorithm but also an idea for computer transcendental functions like 
Sine, Cosine, and Tangent. These functions could already be solved by using
taylor series as its commonly known that 

$$
\sinh(x) = \sum_{n=0}^{\inf} \frac{(-1)^n * x^{2n+1}}{(2n+1)!} = x - \frac{x^3}{3!} + \frac{x^5}{5!} + ..
$$
$$
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
        |/ t    |                                                               
        ------------->-----                                                     
        |   A        1                                                          
        |                                                                       
        |                                                                       
        |                                                                       
        |                                                                       
        |                                                                       
      -pi/2                                                                                                                                                    
        
Let C be a line of length 1 that terminates on the unit circle. Than we can say that
(A,B) = (cos($\theta$), sin($\theta$)). So the game is on than if we can generate C we have
both cosine and sine computed but the question becomes how? Well the solution is
we are going to rotate V(x,y) = [1, 0] by angle $\theta$ which is known. This can be
write in matrix forum like 

$$
    \begin{bmatrix}
    A \\
    B 
    \end{bmatrix}  =
    \begin{bmatrix}
    cos(\theta) & - sin(\theta) \\
    sin(\theta) & cos(\theta) 
    \end{bmatrix}
    \begin{bmatrix}
    1 \\ 
    0
    \end{bmatrix}
$$

You might be seeing a problem if we don't know cos($\theta$) generating C this way
seems well kind of impossible. If we don't know cos($\theta$) what if we knew a subset
of of rotations and just apply them in series to get closer and closer to a full
rotation of $\theta$ radians. Essential we are going to binary seach here and at
each step we know our starting angle $\theta_i$  (i=0 implies 0). If the
starting angle is less than our target $\theta$ we need to rotate left more and
apply a positive rotation. Otherwise a negative rotation. 

This way even if we can't predict the exact rotation we need, we
can pick a set of rotations that approach and in a sense binary search for
$\theta$. But first lets simply our matrix a little bit by factoring out sin
from our matrix.

$$
    \begin{bmatrix}
    cos(\theta) & - sin(\theta) \\
    sin(\theta) & cos(\theta) 
    \end{bmatrix} = 
    cos(\theta) \begin{bmatrix}
    1 & - tan(\theta) \\ 
    tan(\theta) & 1 
    \end{bmatrix}
$$

This splits our problem into two pieces a rotation and than a scalar multiply
which we can calculate seperately. So for our binary search we need to select a
series of angles but what set? Well we could chose in degrees $45/2^i$ for each
iterator 0..15 (we are terminating early but you can go on for more accuracy)
The series would be able to converge on $\theta$ but at each step we require a
multiply and while we can use a lookup table we don't need to. 
What if we picked $\theta_i$ such that $tan(\theta_i) = \frac{1}{2^i}$. This
would allow us to simply use a shift instead of a multiply which should be
faster in hardware. Futhermore since tangent is an odd function (symmetric about
the y axis) if have to rotate negative we just flip the sign of the tangents.
Let the results of our binary search than be call $s_i$ 

Now while we are at we can't forget that we also have these scalar $cos(\theta_i)$
At each step we can pull the term out and simply multiply but the final product
because scalar's times matrix is communitive. I.e $\prod_{n=1}^{15} cosine(theta_i)$ 
This probably seems unfair to start because we said we wanted to calculate sine
and cosine with cordic. Even though these are fixed we can calculate them
without knowing the cosine values by applying the following identity and
calculating. 

$$
cosine(\theta_i) = \frac{1}{\sqrt{1 + tan^2 (\theta_i)}} \\
K = \prod_{n=1}^{16} \frac{1}{\sqrt(1 + 2^{-2i})} = 0.6072529351
$$

I also want to save the square of this value which for these iterations was 0.3687561272.
The reason why we be come clear in a bit. 

Now lets list the angles we are going to use to converge to our target $\theta$
which are as follows

$$
\begin{bmatrix}
i & \theta_i & deg(\theta_i) & tan (\theta_i) \\
0 & 0.7853981634 & 45 & \frac{1}{2^i} \\
1 & 0.463647609 & 26.56505118 & \frac{1}{2^i} \\
2 & 0.2449786631 & 14.03624347 & \frac{1}{2^i} \\
3 & 0.1243549945 & 7.125016349 & \frac{1}{2^i} \\
4 & 0.06241881 & 3.576334375 & \frac{1}{2^i} \\
5 & 0.03123983343 & 1.789910608 & \frac{1}{2^i} \\
6 & 0.01562372862 & 0.8951737102 & \frac{1}{2^i} \\
7 & 0.00781234106 & 0.4476141709 & \frac{1}{2^i} \\
8 & 0.003906230132 & 0.2238105004 & \frac{1}{2^i} \\
9 & 0.001953122516 & 0.1119056771 & \frac{1}{2^i} \\
10 & 0.0009765621896 & 0.05595289189 & \frac{1}{2^i} \\
11 & 0.0004882812112 & 0.02797645262 & \frac{1}{2^i} \\
12 & 0.0002441406201 & 0.01398822714 & \frac{1}{2^i} \\
13 & 0.0001220703119 & 0.006994113675 & \frac{1}{2^i} \\
14 & 0.00006103515617 & 0.003497056851 & \frac{1}{2^i} \\
15 & 0.00003051757812 & 0.001748528427 & \frac{1}{2^i} \\
\end{bmatrix}
$$

So we can than calculate (x_{i+1}, y_{i+1}, $u_{i+1}$)CordicStep($x_i$, $y_i$, $u_i$, i)

$$
\begin{bmatrix}
x_{i+1} \\ 
y_{i+1}
\end{bmatrix} = 
\begin{bmatrix}
1 & - s_i * \frac{1}{2^i} \\ 
s_i \frac{1}{2^i} & 1 
\end{bmatrix} *
\begin{bmatrix}
x_i \\ 
y_i
\end{bmatrix}
$$

After each iteration we should have have the parts for that next iteration and
after every iteration we should be able to return 
($K*x_n$= cos($\theta$), $K*y_n$ = sin($\theta$)). Which is exactly what we are 
looking for with a catch. Our rotation input only works for values on the right
side of the y-axis. This is fine if we want to invert x inputs and catch the 
+/- 90 degrees solution however we can get more percision and handle all inputs
by simply running 2 iterations of the cordic step function before incrementing
i. This will improve convergence and works for all arguments modulo 2$\pi$. So
we are done right? Well algorithmically yes but to implement this we are going
to not use floating point because that is over kill we only need intergers
arithmtic to do this. 

# What is fixed point math?
Fixed point is just normal integer math on an ALU however what changes is how we
interpert intermediate values. The normal way of look at a binary number is to
say that if it is n bits long it can represent either 0 to $2^n$ or it can
represent $-2^{n-1} - 1$ to $2^{n-1}$. These are the normal representations but
they are not the only representations / mental models we can have. If n = 32 a person
might say this number system is A(31,0). I.e it signed 32 bits but bits do not
represent a fractional of of a whole number but there is no reason we can say we
want to count every half step or quarter step of whole numbers. I.e lets say we
split it evenly than we have A(M,N) = A(15,16) i.e Q16 because its signed
rationals. Here we represent $2^M -1$ to $2^N$ in steps of $\frac{1}{2^N}$. 

Thus to represent 1 in Q16 our encoding would be 0x00010000. But you could
also represent 1.5 as the halfway point which would be 0x00018000. The smallest
positive value you can represent is 0x00000001 = \frac{1}{2^16} = 0.0000152587890625 

# Where to start when implementing 

I think a plan or a record of how I came to implementing the algorithm might be
useful. 

When I was implementing this algorithm I first started with a Fxp type that was
a A(15,16) signed type and making a fixed2float and float2fixed function to get
the easy decimal printing noting that when you convert between the two you might
get signficant noise error! Noise is something I haven't mentioned to much but
when working with decmials if an expression requires more bits of accuracy below
or above than are available you are going to get an ever so slightly incorrect
value. This matters when you implement algorithms with more and more additions
and multiplications etc. Fixed point gets complex I might make a follow on when
I feel confident in explaing it fully. 

After I had these types I implemented  the * / + - operators in every fasion in
rust. I am implementing these on a x86 machine which means that my 64 bit
registers are essentially free so rather than implementing smart 32 bit
multiplications and additions I simply upcasted and did standard operations
before shifting back into range for my expression. For mutliplication it would (A
* B) >> 16 to scale back down. Why scale back down well, If you are
going to multiply 2 numbers by a * b and they can be $2^16$ than your resultant
will be $2^32$ which is not the correct range for the resultant. Thus we scall
back down. Note this does not prevent overflow it just for selecting the correct
range for your representation. If you overflow you have a big issue. Divison
works similar but like   ( a << 16 ) / b. This is because b could be $2^16$ and
if we divide by it our fixed point scale will be showing the wrong thing. 

Finally I implemented Cordic, But I started with some constant tables, you can
use the ones I made but you can also use your float2fixed function to generate
the tables for you like I did. After I made the Cordic function (my original
implmentation didn't have the Cordic step broken out). Fold works really well in
rust for this function since you use the Cordic angles one time for each time.
The iter step works well as ($\theta$, $x_0$, $y_0$, $\theta_0$, $\theta_1$, i)
-> ($x_i$, $y_i$, $theta_i$) passing i helps because you should be shifting by
the stage amount for multiply instead of actually multiplying. 

A full implementation shouldn't take more than 30 lines if you get the
intermediate values passed correctly. 

# Extra goals 
After I did I implemented a taylor expansion with my fixed point type which
quickly had overflow issues but could estimate with at most 8 terms before
overflowing but 7 was more stable. The difference between the two was very
marginal from actual cosine with 16 iterations I had 4 bits of accuacy. I read
online that Hardware will usually do 40 iterations after scaling inputs but will
gain around 1 bit of accuracy per iteration. I would need to expand my fixed
point representation to support such accuracy but I felt content after getting 5
bits of accuracy. 

# Verilog
I didn't do synthesis because I lacked a FPGA and verilog experience but I
wanted to try verilog genvar keyword out because it looked so elegant and lisp
like. To do the code gen? hw gen I used yosys and wrote a neat makefile for
generating cxxrtl backends + testbench checking for. Ideally i would have done
the cxx in rust but that seemed like something i didn't want to do at this time.
The Verilog solution also uses Q8.16 which uses less flops  than Q16.16



