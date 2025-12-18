# sha256sum-rs

Uma ferramenta CLI rápida e segura para criar o 'sha256sum' de um ficheiro ou qualquer conteúdo via 'stdin'.

---

## 🚀 Utilização (CLI)

### Exemplo com pipe

```bash
echo -n "hello" | sha256sum-rs
```

### Exemplo com um ficheiro

```bash
sha256sum-rs ficheiro.txt > ficheiro.txt.sha256
```
