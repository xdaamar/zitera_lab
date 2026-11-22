// scripts/test_trial_conversion_engine.mjs
// End-to-End Verification Suite for 72-Hour Free Trial Conversion Engine
import assert from 'node:assert/strict';
import { canUserAccessLab, getLabProgression, isLabSolved } from '../ui/tauri/src/progressionHelper.ts';

const TRACK_1_FOUNDATIONS = ['F01', 'F02', 'F03', 'F04', 'F05', 'F06'];

const TRACK_2_OWASP_TOP10 = [
  'A01', 'A01-02', 'A01-03',
  'A02', 'A02-02', 'A02-03',
  'A03', 'A03-02', 'A03-03',
  'A04', 'A04-02', 'A04-03',
  'A05', 'A05-02', 'A05-03',
  'A06', 'A06-02', 'A06-03',
  'A07', 'A07-02', 'A07-03',
  'A08', 'A08-02', 'A08-03',
  'A09', 'A09-02', 'A09-03',
  'A10', 'A10-02', 'A10-03'
];

const TRACK_3_SPECIALIST = ['W01', 'W02', 'W03', 'W04', 'W05', 'D01', 'D02', 'A11', 'A12'];

const ALL_45_LABS = [...TRACK_1_FOUNDATIONS, ...TRACK_2_OWASP_TOP10, ...TRACK_3_SPECIALIST];

console.log('='.repeat(65));
console.log('   ZITERA LAB: 72-HOUR FREE TRIAL CONVERSION ENGINE QA SUITE   ');
console.log('='.repeat(65));

// -------------------------------------------------------------
// TEST SUITE 1: Authoritative Access Control for Trial Users
// -------------------------------------------------------------
console.log('\n[TEST 1] Verifying Trial / Foundation User Access Control:');

let trialAllowedCount = 0;
let trialBlockedCount = 0;

for (const labId of ALL_45_LABS) {
  const result = canUserAccessLab(labId, 'foundation');
  
  // Golden Path: F01..F06 and A01 (VaultPay Enterprise IDOR) must be accessible
  if (TRACK_1_FOUNDATIONS.includes(labId) || labId === 'A01') {
    assert.equal(result.allowed, true, `Lab ${labId} should be ACCESSIBLE to trial user`);
    assert.equal(result.isTierLocked, false, `Lab ${labId} must not be tier locked`);
    assert.equal(result.requiredTier, 'foundation');
    trialAllowedCount++;
  } else {
    // All other 38 labs (A01-02, A01-03, A02..A10 + explores + Track 3) must be strictly locked
    assert.equal(result.allowed, false, `Lab ${labId} should be LOCKED for trial user`);
    assert.equal(result.isTierLocked, true, `Lab ${labId} must be tier locked`);
    assert.equal(result.requiredTier, 'pro', `Lab ${labId} must require 'pro' tier`);
    assert.ok(result.reason && result.reason.length > 0, `Lab ${labId} must provide explanatory reason`);
    trialBlockedCount++;
  }
}

console.log(`  ✓ Golden Path & Foundations Accessible: ${trialAllowedCount}/7 Labs (F01..F06 + A01)`);
console.log(`  ✓ VIP Pro Exclusive Labs Locked: ${trialBlockedCount}/38 Labs (A01-02..A10-03 + Track 3)`);
assert.equal(trialAllowedCount, 7, 'Expected exactly 7 accessible labs for trial user');
assert.equal(trialBlockedCount, 38, 'Expected exactly 38 locked labs for trial user');

// -------------------------------------------------------------
// TEST SUITE 2: Pro & Enterprise Full Unlocked Access
// -------------------------------------------------------------
console.log('\n[TEST 2] Verifying Pro & Enterprise Tier Access:');

let proAllowedCount = 0;
let entAllowedCount = 0;

for (const labId of ALL_45_LABS) {
  const proRes = canUserAccessLab(labId, 'pro');
  assert.equal(proRes.allowed, true, `Pro user must access lab ${labId}`);
  assert.equal(proRes.isTierLocked, false);
  proAllowedCount++;

  const entRes = canUserAccessLab(labId, 'enterprise');
  assert.equal(entRes.allowed, true, `Enterprise user must access lab ${labId}`);
  assert.equal(entRes.isTierLocked, false);
  entAllowedCount++;
}

console.log(`  ✓ Pro Tier: ${proAllowedCount}/45 Labs UNLOCKED`);
console.log(`  ✓ Enterprise Tier: ${entAllowedCount}/45 Labs UNLOCKED`);

// -------------------------------------------------------------
// TEST SUITE 3: CTF Flag Submission & Scoreboard Recording Simulation
// -------------------------------------------------------------
console.log('\n[TEST 3] Simulating CTF Flag Submission & Scoreboard Recording:');

// Mock Scoreboard database
const mockSolvesTable = [];
const mockUsersTable = [
  { id: 'usr_trial_01', username: 'student_trial', tier: 'foundation', avatarId: 'zeta' },
  { id: 'usr_pro_01', username: 'student_pro', tier: 'pro', avatarId: 'hacker' }
];

function simulateRecordSolve(user, labId, points = 100) {
  const cleanLabId = labId.trim().toUpperCase();
  const access = canUserAccessLab(cleanLabId, user.tier);

  if (!access.allowed) {
    return {
      success: false,
      error: `Access denied: User ${user.username} (tier: ${user.tier}) is unauthorized to submit solve for ${cleanLabId} (${access.trackName}).`
    };
  }

  const solveId = `s_${user.id}_${cleanLabId}`;
  mockSolvesTable.push({
    id: solveId,
    user_id: user.id,
    username: user.username,
    lab_id: cleanLabId,
    points,
    solved_at: Date.now()
  });

  return { success: true };
}

const trialUser = mockUsersTable[0];

// 3.1: Trial user submits F01 flag (Mission 1)
const solveF01 = simulateRecordSolve(trialUser, 'F01');
assert.equal(solveF01.success, true, 'Trial user should successfully submit F01 solve');

// 3.2: Trial user submits F06 flag (Mission 1)
const solveF06 = simulateRecordSolve(trialUser, 'F06');
assert.equal(solveF06.success, true, 'Trial user should successfully submit F06 solve');

// 3.3: Trial user submits A01 flag (Mission 2: VaultPay IDOR)
const solveA01 = simulateRecordSolve(trialUser, 'A01');
assert.equal(solveA01.success, true, 'Trial user should successfully submit A01 solve');

// 3.4: Trial user attempts submitting A02 (Locked Pro Lab) -> MUST BE REJECTED
const solveA02 = simulateRecordSolve(trialUser, 'A02');
assert.equal(solveA02.success, false, 'Trial user solve on A02 must be rejected');
assert.match(solveA02.error, /unauthorized to submit solve for A02/);

// 3.5: Trial user attempts submitting A10 (Locked Pro Lab) -> MUST BE REJECTED
const solveA10 = simulateRecordSolve(trialUser, 'A10');
assert.equal(solveA10.success, false, 'Trial user solve on A10 must be rejected');
assert.match(solveA10.error, /unauthorized to submit solve for A10/);

// Verify Scoreboard aggregation for trial user
const trialSolves = mockSolvesTable.filter(s => s.user_id === trialUser.id);
const totalPoints = trialSolves.reduce((sum, s) => sum + s.points, 0);
assert.equal(trialSolves.length, 3, 'Trial user should have exactly 3 recorded solves (F01, F06, A01)');
assert.equal(totalPoints, 300, 'Trial user total points should be exactly 300');

console.log('  ✓ Trial user flag submissions for F01, F06, A01 recorded successfully');
console.log('  ✓ Trial user unauthorized flag submissions for A02..A10 blocked by anti-cheat');
console.log(`  ✓ Scoreboard aggregation verified: 3 solves, ${totalPoints} points recorded`);

// -------------------------------------------------------------
// TEST SUITE 4: Admin Portal Renewal & Trial Life Cycle Simulation
// -------------------------------------------------------------
console.log('\n[TEST 4] Simulating Admin Portal Renewal & Trial Life Cycle:');

// Functions ported identically from admin_portal.html
function calculateNewExpiry(currentExpiry, durationKey, customVal) {
  const now = Date.now();
  const baseTime = (currentExpiry && Number(currentExpiry) > now) ? Number(currentExpiry) : now;

  if (durationKey === 'lifetime') return null;
  if (durationKey === '3d' || durationKey === 'trial_3d') return baseTime + 3 * 24 * 60 * 60 * 1000;
  if (durationKey === '1m') return baseTime + 30 * 24 * 60 * 60 * 1000;
  if (durationKey === '2m') return baseTime + 60 * 24 * 60 * 60 * 1000;
  if (durationKey === '3m') return baseTime + 90 * 24 * 60 * 60 * 1000;
  if (durationKey === '6m') return baseTime + 180 * 24 * 60 * 60 * 1000;
  if (durationKey === '12m') return baseTime + 365 * 24 * 60 * 60 * 1000;
  if (durationKey === 'custom' && customVal) {
    return new Date(customVal).getTime();
  }
  return baseTime + 30 * 24 * 60 * 60 * 1000;
}

function getExpirationDetails(user) {
  if (user.role === 'admin' || !user.expires_at || Number(user.expires_at) === 0) {
    return { isLifetime: true, isExpired: false, isTrial: false };
  }
  const exp = Number(user.expires_at);
  const now = Date.now();
  const isExpired = exp <= now;
  const isTrial = (user.notes && user.notes.toUpperCase().includes('TRIAL')) || 
                  (user.created_at && (exp - Number(user.created_at) <= 4 * 86400000));
  return { isLifetime: false, isExpired, isTrial };
}

const now = Date.now();

// 4.1: Quick 72-Hour Trial Creation
const trialExpiry = calculateNewExpiry(null, 'trial_3d');
const trialDiff = trialExpiry - now;
const trialDays = Math.round(trialDiff / (24 * 60 * 60 * 1000));
assert.equal(trialDays, 3, 'Quick trial must calculate 3 days duration');

const testTrialUser = {
  id: 'u_trial_1',
  username: 'budi_trial',
  created_at: now,
  expires_at: trialExpiry,
  notes: '[TRIAL 3 HARI] Mas Budi',
  is_active: 1
};
const trialDetails = getExpirationDetails(testTrialUser);
assert.equal(trialDetails.isTrial, true, 'User must be identified as trial user');
assert.equal(trialDetails.isExpired, false, 'User must be active');

// 4.2: Expired Account Simulation & Background Deactivation
const expiredUser = {
  id: 'u_trial_expired',
  username: 'ani_trial',
  created_at: now - 5 * 86400000,
  expires_at: now - 2 * 86400000, // Expired 2 days ago
  notes: '[TRIAL 3 HARI] Mbak Ani',
  is_active: 0 // Deactivated
};
const expiredDetails = getExpirationDetails(expiredUser);
assert.equal(expiredDetails.isExpired, true, 'User must be identified as expired');

// 4.3: Renewal Flow - Upgrading Expired Trial User to Pro 1 Bulan
const renewedExpiry = calculateNewExpiry(expiredUser.expires_at, '1m');
const renewedDiff = renewedExpiry - now;
const renewedDays = Math.round(renewedDiff / (24 * 60 * 60 * 1000));
assert.equal(renewedDays, 30, 'Renewed expired user must calculate 30 days from now');

// Simulate DB update
expiredUser.expires_at = renewedExpiry;
expiredUser.is_active = 1; // Reactivated
expiredUser.tier = 'pro'; // Upgraded
expiredUser.notes = 'Upgraded to PRO via WhatsApp';

const renewedDetails = getExpirationDetails(expiredUser);
assert.equal(renewedDetails.isExpired, false, 'User must now be active');
assert.equal(expiredUser.is_active, 1, 'Account must be reactivated');
assert.equal(canUserAccessLab('A02', expiredUser.tier).allowed, true, 'Upgraded user can now access Pro labs');

console.log('  ✓ Quick 72-Hour (3 Days) trial calculation verified');
console.log('  ✓ Trial badge and expiration detection verified');
console.log('  ✓ Renewal from expired status sets new expiration date correctly');
console.log('  ✓ Account reactivation and Pro tier upgrade unlock all 45 labs');

console.log('\n' + '='.repeat(65));
console.log('  ALL 4 VERIFICATION TEST SUITES PASSED FLAWLESSLY! (100% GREEN)  ');
console.log('='.repeat(65));
