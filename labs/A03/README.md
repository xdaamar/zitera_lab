# ZITERA_LAB: Lab A03 — Software Supply Chain Failures

[![OWASP](https://img.shields.io/badge/OWASP-A03%3A2025-blue)](https://owasp.org/Top10/)
[![Difficulty](https://img.shields.io/badge/Difficulty-Beginner-green)](#)
[![Port](https://img.shields.io/badge/Port-8013-orange)](#)

## Overview
This laboratory illustrates software supply chain attacks where modern applications inadvertently ingest poisoned, unpinned, or compromised dependencies from public registries.

## Target Environment
- Local Host URL: `http://127.0.0.1:8013`
- Runtime: Native Sandboxed (`AppContainer + JobObject + stdio broker`)
- Architecture: Zero external network egress, local synthetic package registry audit

## Objectives
1. Inspect the target application's exposed dependency registry and lockfile audit endpoint.
2. Identify an unpinned third-party package containing backdoored configuration data.
3. Extract the leaked pipeline deployment key to recover the CTF flag.
