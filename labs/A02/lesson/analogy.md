# The Factory Safe Analogy

Imagine purchasing a commercial high-security digital safe for an office:

1. **Factory Settings:** When manufactured, the factory sets the master combination code to `0-0-0-0` and prints this combination clearly in the user manual.
2. **The Security Misconfiguration:** The business installs the safe, deposits company gold bars and confidential documents inside, but never changes the default `0-0-0-0` combination code. Furthermore, they leave the service maintenance hatch unlocked on the back of the safe.
3. **The Intrusion:** An intruder does not need an explosive charge, a drill, or advanced safecracking tools. They simply approach the keypad, enter `0-0-0-0`, and the heavy steel door swings wide open.

In software, building advanced cryptographic algorithms or complex authentication mechanisms is meaningless if you leave the default factory password enabled or leave debug maintenance ports open to the public Internet.
