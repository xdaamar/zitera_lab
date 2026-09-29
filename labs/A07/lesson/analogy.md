# Real-World Analogy: The Briefcase Lock

Imagine a high-security office that locks its front entrance with a cheap 3-digit combination briefcase lock. 

There are only 1,000 possible combinations (000 to 999). If a burglar walks up to the lock and manually spins the wheels, they can try a combination every second. Within 15 minutes, they are guaranteed to open the lock.

If the lock has no guard, no alarm, and no mechanism that freezes the dials after 3 wrong tries, it offers virtually zero real security.

In web applications, a login form without rate-limiting, CAPTCHA, or lockout is like that unguarded combination lock. Attackers write automated scripts that can test thousands of guesses every second until they break in.
