### Generate cert
```openssl req -newkey rsa:2048 -nodes -keyout tls/key.pem -x509 -days 730 -out tls/cert.pem -subj '/CN=localhost'```