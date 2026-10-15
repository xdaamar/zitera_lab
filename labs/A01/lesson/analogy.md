# Real-World Analogy: The Hotel Keycard & Luggage Claim

To understand why Broken Access Control happens intuitively, imagine checking into a modern hotel:

---

### Phase 1: Authentication at the Front Desk
1. You arrive at the hotel lobby and present your government ID and reservation confirmation to the receptionist.
2. The receptionist verifies your identity in the hotel database and confirms your booking.
3. Once your identity is verified (*Authentication Succeeded*), the front desk gives you an electronic keycard labeled: **Room #302**.

---

### Phase 2: Proper Access Control (The Ideal System)
- You take the elevator to the 3rd floor and tap your keycard against Room #302. The electronic lock turns green, clicks, and unlocks.
- If you walk over to **Room #303** rented by another guest and tap your card, the lock flashes red and remains securely locked.
- The lock independently verifies whether your card's encrypted chip holds permission to enter Room #303 (*Authorization Successfully Enforced*).

---

### Phase 3: Broken Access Control (The Vulnerable Scenario)
Now imagine the hotel installs an over-engineered digital lock system with a catastrophic architectural flaw:
1. Whenever you stand in front of any door, the lock queries your smartphone app:
   `"Which room number do you wish to unlock?"`
2. Your phone app initially displays `302`.
3. However, if you manually edit the field on your phone to **`303`** or **`101`** (the Presidential Suite), the lock clicks and swings open!

---

### The Security Takeaway
The hotel front desk successfully verified who you were (you are indeed a registered guest), but the door lock made the **fatal mistake** of trusting the room number submitted by your client application without cross-checking the central reservation ledger to ensure you were the legitimate occupant of that room.

This is the exact essence of **Insecure Direct Object References (IDOR)**:
The web application knows you are a valid authenticated user (*Alice is logged in*), but the server carelessly permits you to read other users' confidential data (*Bob's invoice #2*) simply because you altered an integer parameter in the browser's address bar.
