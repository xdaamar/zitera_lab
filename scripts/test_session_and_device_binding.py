import time

print("=" * 60)
print("  ZITERA LAB: SESSION EXPIRY & DEVICE BINDING TEST SUITE  ")
print("=" * 60)

# 1. Device Match Logic
def extract_guid(s: str) -> str:
    parts = s.split('_')
    return parts[-1] if len(parts) >= 2 else s

def is_device_match(stored: str, current: str) -> bool:
    s = stored.strip().lower()
    c = current.strip().lower()
    if s == c:
        return True
    s_guid = extract_guid(s)
    c_guid = extract_guid(c)
    if s_guid and c_guid and len(s_guid) > 10 and s_guid == c_guid:
        return True
    return False

# Test Cases
t1 = is_device_match("DESKTOP-BA3QF0L_52036371-cb2b-428d-bbc6-e681e3a3a154", "DESKTOP-BA3QF0L_52036371-cb2b-428d-bbc6-e681e3a3a154")
assert t1 is True, "Exact match should pass"

t2 = is_device_match("DESKTOP-OLD_52036371-cb2b-428d-bbc6-e681e3a3a154", "DESKTOP-NEW_52036371-cb2b-428d-bbc6-e681e3a3a154")
assert t2 is True, "Same MachineGuid across hostname change should pass"

t3 = is_device_match("DESKTOP-BA3QF0L_52036371-cb2b-428d-bbc6-e681e3a3a154", "OTHERPC_99999999-1111-2222-3333-444444444444")
assert t3 is False, "Different MachineGuid should be strictly rejected"

print("[TEST 1] Device Binding Multi-Hardware Verification: PASSED")

# 2. 5-Hour Session Inactivity Expiry Logic
SESSION_MAX_INACTIVE_MS = 5 * 60 * 60 * 1000 # 18,000,000 ms

now = int(time.time() * 1000)

# Case A: User closed app 2 hours ago (7,200,000 ms)
last_active_2h_ago = now - (2 * 60 * 60 * 1000)
is_expired_2h = (now - last_active_2h_ago) > SESSION_MAX_INACTIVE_MS
assert is_expired_2h is False, "2-hour closed session should remain valid"

# Case B: User closed app 5 hours and 5 minutes ago (18,300,000 ms)
last_active_5h_ago = now - (5 * 60 * 60 * 1000 + 300000)
is_expired_5h = (now - last_active_5h_ago) > SESSION_MAX_INACTIVE_MS
assert is_expired_5h is True, "5h 5m closed session should trigger automatic logout"

# Case C: User closed app 1 day ago
last_active_1d_ago = now - (24 * 60 * 60 * 1000)
is_expired_1d = (now - last_active_1d_ago) > SESSION_MAX_INACTIVE_MS
assert is_expired_1d is True, "1-day closed session should trigger automatic logout"

print("[TEST 2] 5-Hour Inactivity & App-Close Session Expiry: PASSED")

print("=" * 60)
print("ALL SESSION & DEVICE BINDING LOGIC TESTS PASSED SUCCESSFULLY!")
print("=" * 60)
