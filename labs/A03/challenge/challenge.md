# CTF Challenge: Software Supply Chain Compromise

## Scenario
ApexCorp builds their software product using automated CI/CD dependency resolution. An adversary managed to publish a backdoored package (`apex-internal-telemetry`) targeting their build environment.

The pipeline completed an audit report but continued deploying because signature enforcement was disabled.

## Objective
1. Inspect the target application running at `http://127.0.0.1:8013`.
2. Locate the exposed dependency lockfile audit endpoint.
3. Identify the unpinned, untrusted third-party package.
4. Extract the leaked pipeline deployment authorization token (the flag) and submit it below.
