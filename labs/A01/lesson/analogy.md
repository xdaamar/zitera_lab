# The Hotel Keycard Analogy

Imagine checking into a hotel:

1. **Authentication:** You show your ID card at the front desk. The receptionist verifies you have a reservation and hands you room keycard `#302`.
2. **Access Control (Ideal):** Your keycard only unlocks door `#302`. If you walk to door `#303` and tap your card, the lock flashes red and remains locked.
3. **Broken Access Control (Vulnerability):** Suppose the hotel uses an insecure digital lock where tapping your keycard asks your phone app: *"Which room do you want to unlock?"* If you edit the app to say `303` instead of `302`, the door clicks open!

The front desk verified who you were (Authentication succeeded), but the room door trusted your client input without checking whether room 303 was actually rented to you (Authorization failed).
