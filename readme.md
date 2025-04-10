# email-sanitizer by SelfSend

A modern, scalable, and secure API built with future-ready technologies. Designed for high performance and ease of maintenance.

## 🚀 Features

- **Scalable Architecture**: Built with and optimized for horizontal scaling.
- **Real-Time Support**: Optional WebSocket/SSE endpoints for real-time communication.
- **Security First**: OAuth2/JWT authentication, rate limiting, and input validation.
- **Observability**: Integrated logging, metrics, and distributed tracing.
- **Multi-Protocol Support**: REST, GraphQL, or gRPC endpoints (choose as needed).

## 🛠 Tech Stack

| Category       | Tools                                   |
| -------------- | --------------------------------------- |
| **Language**   | Rust                                    |
| **Framework**  | Actix                                   |
| **Database**   | PostgreSQL + Redis (caching)            |
| **Infra**      | AWS Lambda/Kubernetes + Terraform (IaC) |
| **Auth**       | Auth0/Clerk/PASETO/OAuth2               |
| **Monitoring** | Prometheus + Grafana, ELK Stack         |

## 📦 Getting Started

### Prerequisites

- Rust 1.65+
- PostgreSQL/Redis (or Docker)
- Terraform (optional, for cloud provisioning)

### Installation

1. Clone the repo:
   ```bash
   git clone https://github.com/self-send/email-sanitizer.git
   ```
2. Install Dependencies:
   ```bash
   cargo build
   ```

### 🔧 Environment Setup

Configure your `.env` file:

```bash
  PORT=3000
  DATABASE_URL=postgresql://user:pass@localhost:5432/db
  REDIS_URL=redis://localhost:6379
  JWT_SECRET=your_secure_secret
  API_RATE_LIMIT=100
```

### 🏗️ Running the Server

```bash
# Development (hot-reload)
cargo watch -x run

# Production build
cargo build --release
```

### 🚨 Contributing

- Fork the repository.
- Create a branch:

```bash
git checkout -b feat/issue-number-issue-name #9-set-up-rust-project-with-actixaxum
```

- Follow Conventional Commits.
- Submit a PR with tests and documentation.

### 📄 License

MIT License.

### 🌟 Future Roadmap

#### **Email Sanitization API Roadmap (Rust, PostgreSQL, Redis, REST & GraphQL)**

##### **Phase 1: Core Setup & Validation (Sprint 1-2)**

###### **Tasks**

1. **Project Initialization**

   - Set up Rust project with Actix/Axum.
   - Configure CI/CD (GitHub Actions).
   - **DoD**: Project builds successfully, CI pipeline passes.

2. **Basic Email Validation**

   - Implement syntax validation (regex).
   - Add DNS/MX record verification.
   - **DoD**: Unit tests cover 90% of cases, returns structured validation results.

3. **PostgreSQL Integration**

   - Design database schema for storing validation results.
   - Implement basic CRUD operations.
   - **DoD**: DB migrations applied, test queries succeed.

4. **REST API (Basic Endpoints)**
   - Implement `POST /validate` for single email validation.
   - Add error handling and OpenAPI docs.
   - **DoD**: Endpoint tested via Postman, Swagger UI works.

---

##### **Phase 2: GraphQL & Advanced Features (Sprint 3-4)**

###### **Tasks**

5. **GraphQL Integration**

   - Set up GraphQL server (Async-GraphQL/Juniper).
   - Add `validateEmail` query and `validateBulkEmails` mutation.
   - **DoD**: GraphQL playground accessible, queries return correct responses.

6. **Redis Caching Layer**

   - Cache DNS/MX results to reduce latency.
   - Implement TTL for cached entries.
   - **DoD**: Cached responses are 50% faster than uncached ones.

7. **Disposable & Role-Based Email Detection**

   - Integrate blocklists for disposable emails.
   - Detect role-based addresses (e.g., `admin@`, `support@`).
   - **DoD**: Blocklists loaded at startup, role detection accuracy >95%.

8. **Bulk Processing**
   - Add async bulk validation endpoint (`POST /bulk/validate`).
   - Implement job queue (Redis or PostgreSQL).
   - **DoD**: Processes 10K emails in <5 mins, returns job status.

---

##### **Phase 3: Performance & Security (Sprint 5-6)**

###### **Tasks**

9. **Rate Limiting**

   - Implement Redis-based rate limiting (per API key).
   - **DoD**: Rejects requests beyond 10 reqs/sec, logs violations.

10. **Authentication & Authorization**

    - Add JWT/API key authentication.
    - Restrict sensitive endpoints.
    - **DoD**: Unauthorized requests blocked, keys validated via DB.

11. **Monitoring & Logging**

    - Add Prometheus metrics (`/metrics`).
    - Structured logging (`tracing`).
    - **DoD**: Metrics visible in Grafana, logs searchable in Kibana.

12. **Load Testing & Optimization**
    - Benchmark with `k6` (target: 50K RPS).
    - Optimize DB queries and Redis usage.
    - **DoD**: Latency <100ms at 10K RPS, no memory leaks.

---

##### **Phase 4: Deployment & Maintenance (Sprint 7+)**

###### **Tasks**

13. **Docker & Kubernetes Deployment**

    - Containerize API with multi-stage Dockerfile.
    - Deploy to Kubernetes (EKS/GKE) or serverless (AWS Lambda).
    - **DoD**: API runs in production, health checks pass.

14. **Blue-Green Deployment**

    - Zero-downtime deployment strategy.
    - Rollback mechanism.
    - **DoD**: Deploys without downtime, rollback tested.

15. **Post-Launch Monitoring**
    - Set up alerts (Prometheus Alertmanager).
    - Track bounce rate improvements.
    - **DoD**: Alerts trigger on errors, sender score improves by 10%.

---

###### **Roadmap Timeline**

| Sprint | Focus Area         | Key Deliverables           |
| ------ | ------------------ | -------------------------- |
| 1-2    | Core Validation    | REST API, DB integration   |
| 3-4    | GraphQL & Caching  | Bulk processing, Redis     |
| 5-6    | Security & Scaling | Rate limits, auth, 50K RPS |
| 7+     | Deployment         | Kubernetes, monitoring     |

###### **Success Metrics**

- **Performance**: 99.9% uptime, <100ms latency.
- **Accuracy**: 95%+ valid/invalid email detection.
- **Security**: No critical CVEs, rate limits enforced.
