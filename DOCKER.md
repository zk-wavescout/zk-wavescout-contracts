# Docker Setup Guide

## Quick Start with Docker Compose

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+

### 1. Clone with Submodules
```bash
git clone --recurse-submodules https://github.com/zk-wavescout/zk-wavescout-contracts.git
cd zk-wavescout-contracts
```

### 2. Configure Environment
```bash
cp .env.example .env
# Edit .env with your configuration
```

### 3. Start All Services
```bash
docker-compose up -d
```

This starts:
- **Soroban Local Network** (port 8000)
- **ZK Circuits** (Noir compiler)
- **Smart Contract** (builds WASM)
- **Coordinator** (event listener, port 3000)
- **Frontend** (React UI, port 3001)

### 4. Verify Services Running
```bash
docker-compose ps
```

Expected output:
```
NAME                    STATUS
soroban-local           Up 2 minutes
zk-wavescout-circuits   Up 2 minutes
zk-wavescout-contract   Up 2 minutes
zk-wavescout-coordinator Up 1 minute
zk-wavescout-frontend   Up 1 minute
```

### 5. Access Services
- **Soroban RPC:** http://localhost:8000
- **Coordinator API:** http://localhost:3000
- **Frontend UI:** http://localhost:3001

### 6. Test Integration
```bash
# Check contract compilation
docker-compose logs contract

# Check coordinator events
docker-compose logs coordinator

# Check frontend startup
docker-compose logs frontend
```

### 7. Stop All Services
```bash
docker-compose down
```

### 8. Rebuild Services (after code changes)
```bash
docker-compose up -d --build
```

## Service Details

### Soroban Network
- Local Stellar network for testing
- RPC available at `http://localhost:8000`
- Pre-configured test accounts

### Circuits Service
- Compiles Noir ZK circuits
- Generates proving keys
- Output available to contract at build time

### Contract Service
- Builds Soroban WASM binary
- Runs tests
- Output: `target/wasm32-unknown-unknown/release/zk_wavescout_contracts.wasm`

### Coordinator Service
- Node.js backend
- Listens for `bounty_claimed` events
- Manages GitHub PR automation
- Exposes REST API on port 3000

### Frontend Service
- React WASM prover UI
- Compiles circuits client-side
- Submits proofs to contract
- UI on port 3001

## Troubleshooting

### Services won't start
```bash
docker-compose logs -f
```

### Need to rebuild one service
```bash
docker-compose up -d --build coordinator
```

### Clear all containers and volumes
```bash
docker-compose down -v
docker system prune -a
```

### Check container resource usage
```bash
docker stats
```

## Development Workflow

1. **Modify contract code** → `docker-compose up -d --build contract`
2. **Modify circuits** → `docker-compose up -d --build circuits`
3. **Modify coordinator** → `docker-compose up -d --build coordinator`
4. **Modify frontend** → `docker-compose up -d --build frontend`

Changes auto-reload in development mode.
