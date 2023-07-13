<div align="center">

# 水彩 suisai

**suisai** (watercolor) is a digital photo album

[![GPLv2](https://img.shields.io/badge/license-GPLv2-green)](#)

</div>

### Running for Development
Start SurrealDB

    surreal start --log trace --user root --pass root memory

Start a SurrealDB REPL and initialize the database by pasting the contents of `init_db.srql`

    surreal sql --conn http://0.0.0.0:8000 --user root --pass root --ns test --db test
    
Exit the REPL, and start `suisai`

    cargo run

