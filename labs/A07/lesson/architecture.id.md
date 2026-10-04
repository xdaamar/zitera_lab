# Prinsip Arsitektur: Autentikasi Terpusat & Terisolasi

Lapisan autentikasi harus didelegasikan ke penyedia identitas terpusat (*Centralized Identity Provider / IdP*) yang menerapkan standar industri terbuka seperti OAuth 2.0, OpenID Connect (OIDC), atau SAML.

Hindari membuat mekanisme login dan manajemen token sesi buatan sendiri yang rentan terhadap kebocoran algoritma kriptografi.
