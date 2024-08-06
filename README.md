# Lerpz Backend

```bash
openssl genpkey -algorithm ED25519 -outform PEM -out ./keys/ed25519_private.pem 
```

```bash
openssl pkey -in ./keys/ed25519_private.pem -pubout -out ./keys/ed25519_public.pem
```
