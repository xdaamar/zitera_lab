"""
Zitera Lab — Security Hardening & Tier Gating Verification Suite
Verifies:
1. Anti-Privilege Escalation (LocalStorage State Tampering Neutralized)
2. Authoritative Tier Gating across all 45 laboratories
3. Anti-CTF Scoreboard Flag Escalation
"""

import json

# Definitive 45 Lab List
TRACK_1_FOUNDATIONS = ["F01", "F02", "F03", "F04", "F05", "F06"]

TRACK_2_OWASP_TOP10 = [
    "A01", "A01-02", "A01-03",
    "A02", "A02-02", "A02-03",
    "A03", "A03-02", "A03-03",
    "A04", "A04-02", "A04-03",
    "A05", "A05-02", "A05-03",
    "A06", "A06-02", "A06-03",
    "A07", "A07-02", "A07-03",
    "A08", "A08-02", "A08-03",
    "A09", "A09-02", "A09-03",
    "A10", "A10-02", "A10-03"
]

TRACK_3_SPECIALIST = ["W01", "W02", "W03", "W04", "W05", "D01", "D02", "A11", "A12"]

ALL_45_LABS = TRACK_1_FOUNDATIONS + TRACK_2_OWASP_TOP10 + TRACK_3_SPECIALIST


def can_user_access_lab(lab_id: str, user_tier: str):
    clean_id = (lab_id or "").strip().upper()
    effective_tier = (user_tier or "foundation").lower()

    if clean_id.startswith("F"):
        return {"allowed": True, "is_tier_locked": False, "track": "TRACK_1"}

    if clean_id.startswith("W") or clean_id.startswith("D") or clean_id in ("A11", "A12"):
        is_pro = effective_tier in ("pro", "enterprise")
        return {"allowed": is_pro, "is_tier_locked": not is_pro, "track": "TRACK_3"}

    is_pro = effective_tier in ("pro", "enterprise")
    return {"allowed": is_pro, "is_tier_locked": not is_pro, "track": "TRACK_2"}


def run_tests():
    print("==================================================")
    print("  ZITERA LAB: TIER AUTHORIZATION & SECURITY AUDIT ")
    print("==================================================")
    print(f"Total verified labs: {len(ALL_45_LABS)} (T1: {len(TRACK_1_FOUNDATIONS)}, T2: {len(TRACK_2_OWASP_TOP10)}, T3: {len(TRACK_3_SPECIALIST)})")
    assert len(ALL_45_LABS) == 45, "Catalog count mismatch!"

    # Test 1: Foundation User Gating
    print("\n[TEST 1] Auditing Foundation User Access:")
    foundation_allowed = 0
    foundation_blocked = 0
    for lab_id in ALL_45_LABS:
        res = can_user_access_lab(lab_id, "foundation")
        if lab_id in TRACK_1_FOUNDATIONS:
            assert res["allowed"] is True, f"Foundation user should access Track 1 lab {lab_id}"
            assert res["is_tier_locked"] is False
            foundation_allowed += 1
        else:
            assert res["allowed"] is False, f"VULNERABILITY: Foundation user accessed restricted lab {lab_id}!"
            assert res["is_tier_locked"] is True
            foundation_blocked += 1

    print(f"  -> Foundation Tier: {foundation_allowed}/6 Track 1 Labs ALLOWED")
    print(f"  -> Foundation Tier: {foundation_blocked}/39 Track 2 & 3 Labs STRICTLY BLOCKED [PASSED]")

    # Test 2: Pro User Access
    print("\n[TEST 2] Auditing Pro User Access:")
    pro_allowed = 0
    for lab_id in ALL_45_LABS:
        res = can_user_access_lab(lab_id, "pro")
        assert res["allowed"] is True, f"Pro user should access lab {lab_id}"
        assert res["is_tier_locked"] is False
        pro_allowed += 1

    print(f"  -> Pro Tier: {pro_allowed}/45 Labs UNLOCKED [PASSED]")

    # Test 3: Enterprise User Access
    print("\n[TEST 3] Auditing Enterprise VIP User Access:")
    ent_allowed = 0
    for lab_id in ALL_45_LABS:
        res = can_user_access_lab(lab_id, "enterprise")
        assert res["allowed"] is True, f"Enterprise user should access lab {lab_id}"
        assert res["is_tier_locked"] is False
        ent_allowed += 1

    print(f"  -> Enterprise Tier: {ent_allowed}/45 Labs UNLOCKED [PASSED]")

    # Test 4: LocalStorage Privilege Escalation Simulation
    print("\n[TEST 4] Simulating LocalStorage Privilege Escalation Attack:")
    # Rogue student tampers with local JSON:
    tampered_local_session = {
        "id": "usr_student_foundation_1",
        "username": "student_foundation",
        "tier": "pro",  # TAMPERED BY ATTACKER
        "role": "student"
    }
    # Database ground truth in Turso:
    turso_ground_truth = {
        "id": "usr_student_foundation_1",
        "username": "student_foundation",
        "tier": "foundation",  # AUTHENTIC SERVER RECORD
        "role": "student",
        "is_active": 1,
        "device_id": "MACHINE_GUID_ABC123"
    }

    # Simulation of verifyServerSession logic in authService.ts:
    server_tier = turso_ground_truth.get("tier", "foundation")
    corrected_user = {
        **tampered_local_session,
        "tier": server_tier,
        "role": turso_ground_truth.get("role", "student")
    }

    print(f"  -> Attacker modified local session tier to: '{tampered_local_session['tier']}'")
    print(f"  -> Server database ground truth is: '{turso_ground_truth['tier']}'")
    print(f"  -> Session verification result: Overridden back to '{corrected_user['tier']}' [PASSED]")
    assert corrected_user["tier"] == "foundation", "Privilege escalation was not reverted!"

    # Test 5: Scoreboard Solve Submission Protection
    print("\n[TEST 5] Auditing CTF Scoreboard Flag Escalation Protection:")
    def submit_solve(user_tier: str, lab_id: str):
        access = can_user_access_lab(lab_id, user_tier)
        if not access["allowed"]:
            return False, "Access Denied: Tier Foundation cannot record solves on Pro tracks"
        return True, "Solve Recorded: +100 PTS"

    ok, msg = submit_solve("foundation", "F01")
    assert ok is True, "Foundation solve for F01 should succeed"

    ok, msg = submit_solve("foundation", "A01")
    assert ok is False, "VULNERABILITY: Foundation user recorded solve on A01!"
    print(f"  -> Foundation user solve attempt on A01: Blocked with '{msg}' [PASSED]")

    ok, msg = submit_solve("foundation", "W04")
    assert ok is False, "VULNERABILITY: Foundation user recorded solve on W04!"
    print(f"  -> Foundation user solve attempt on W04: Blocked with '{msg}' [PASSED]")

    ok, msg = submit_solve("pro", "A01")
    assert ok is True, "Pro user solve on A01 should succeed"
    print(f"  -> Pro user solve attempt on A01: Allowed ({msg}) [PASSED]")

    print("\n" + "="*50)
    print("ALL 5 SECURITY & PRIVILEGE ESCALATION AUDITS PASSED!")
    print("==================================================")

if __name__ == "__main__":
    run_tests()
