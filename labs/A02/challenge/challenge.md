# CTF Challenge: The OpsGateway Configuration Leak

### MISSION BRIEF
You are conducting a security audit of a critical internal operations gateway running on `http://127.0.0.1:8012`. Preliminary passive reconnaissance suggests the web deployment team pushed default application configurations and left administrative debug endpoints enabled.

### OBJECTIVE
Identify security misconfigurations on the target system to uncover the administrator's backup configuration archive and retrieve the system activation flag.

### TARGET ENVIRONMENT
- Base URL: `http://127.0.0.1:8012`
- Scope: Host local-only `127.0.0.1:8012`. Do not attack external networks.

### SUBMISSION FORMAT
The flag follows the standard ZITERA format:
`ZITERA{...}`
