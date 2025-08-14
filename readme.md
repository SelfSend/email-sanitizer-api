###### Code Quality:

[![CI](https://github.com/SelfSend/email-sanitizer-api/actions/workflows/ci.yml/badge.svg)](https://github.com/SelfSend/email-sanitizer-api/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/SelfSend/email-sanitizer-api/branch/main/graph/badge.svg)](https://codecov.io/gh/SelfSend/email-sanitizer-api)

# email-sanitizer by SelfSend

A high-performance and secure REST/GraphQL API built with Rust, MongoDB & Redis for cleaning email subscriber lists. Maintains sender reputation by validating, deduplicating, and pruning inactive emails.

## Features

- **Scalable Architecture**: Built with Rust and optimized for horizontal scaling.
- **Security First**: OAuth2/JWT authentication, rate limiting, and input validation.
- **Observability**: Integrated logging, metrics, and distributed tracing.
- **Multi-Protocol Support**: REST, GraphQL, or gRPC endpoints (choose as needed).

## Validations

Detect and handle invalid email addresses before they take some space in your database, cause delivery issues or harm your sender score.

The API can be used to validate email addresses in real-time or in bulk. It can also be used to clean up existing email lists by removing invalid or duplicate addresses.

The API is built with Rust, designed to be fast and efficient, capable of processing thousands of email addresses per second. It can be integrated into existing applications or used as a standalone service.

Currently including multiple edge-cases and validation checks:

### Syntax Validation Checks

| Validation Type               | Description                                                                |
| ----------------------------- | -------------------------------------------------------------------------- |
| **Local-Part Characters**     | Allow letters, digits, specific symbols; quotes required for spaces.       |
| **Quoted Local-Part**         | Balanced quotes and escaping; spaces allowed only within quotes.           |
| **Local-Part Dots**           | Prohibit leading/trailing/consecutive dots unless quoted.                  |
| **Domain Labels**             | Subdomains 1–63 chars; hyphens allowed only mid-label.                     |
| **Domain IP Literals**        | Validate IPv4/IPv6 addresses in brackets (e.g., `[192.168.1.1]`).          |
| **Domain Case Handling**      | Normalize domain to lowercase (case-insensitive).                          |
| **Unicode Support**           | Allow UTF-8 characters in local-part and domain.                           |
| **Unicode Normalization**     | Normalize Unicode to NFC form to avoid duplicates.                         |
| **SMTPUTF8 Compliance**       | Support SMTPUTF8 extension for non-ASCII addresses.                        |
| **Local-Part Length**         | ≤64 octets (after encoding).                                               |
| **Total Address Length**      | ≤254 octets (including local-part, @, domain).                             |
| **Address Comments**          | Reject or strip RFC 5322-style comments.                                   |
| **Obsolete Syntax**           | Disallow deprecated syntax (e.g., folded whitespace).                      |
| **Quoted Escapes**            | Validate backslash-escaped quotes (e.g., `"user\"name"`).                  |
| **Punycode Conversion**       | Convert international domains to Punycode (e.g., `xn--fiqs8s.xn--55qx5d`). |
| **IP Formatting**             | Validate IPv4/IPv6 syntax in domain literals.                              |
| **Domain Literal Brackets**   | Reject IP literals missing brackets (e.g., `user@192.168.1.1`).            |
| **Reserved Domains**          | Block reserved domains (e.g., `localhost`, `test`).                        |
| **Null Addresses**            | Reject empty addresses (e.g., `<>`).                                       |
| **Local-Part Case**           | Preserve case but flag inconsistencies (case-sensitive).                   |
| **Domain Case Normalization** | Always convert domain to lowercase (case-insensitive).                     |

### DNS/MX Records Validation Checks

Follows RFC specifications by checking A/AAAA records if MX records are missing. Checks either MX records exist or direct IP records (A/AAAA) are present

### Disposable Email Address Validation Checks

Checks among a list of 106,543 disposable email domains, the largest database of disposable emails out there, updated daily.

## Tech Stack

| Category       | Tools                                   |
| -------------- | --------------------------------------- |
| **Language**   | Rust                                    |
| **Framework**  | Actix                                   |
| **Database**   | MongoDB + Redis (caching)               |
| **Infra**      | AWS Lambda/Kubernetes + Terraform (IaC) |
| **Auth**       | Auth0/Clerk/PASETO/OAuth2               |
| **Monitoring** | Prometheus + Grafana, ELK Stack         |

## Getting Started

### Prerequisites

- Rust 1.65+
- MongoDB/Redis (or Docker)
- Terraform (optional, for cloud provisioning)

### Installation

1. Clone the repo:
   ```bash
   git clone https://github.com/SelfSend/email-sanitizer-api.git
   ```
2. Install Dependencies:
   ```bash
   cargo build
   ```

### Environment Setup

Configure your `.env` file:

```bash
MONGODB_URI=mongodb+srv://<<username>>:<<password>>@clusterX.*****.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0 # mongodb://192.168.8.136:27017 on local
DB_NAME_TEST=selfsend_test
DB_NAME_PRODUCTION=selfsend_production
DB_DISPOSABLE_EMAILS_COLLECTION=disposable_email_domains

# Redis
REDIS_URL=redis://127.0.0.1:6379
REDIS_CACHE_TTL=86400 # 1 day in seconds
```

### Running the Server

```bash
# Development (hot-reload)
cargo watch -x run

# Production build
cargo build --release
```

### License

MIT License.

### Future Roadmap

##### **Phase 1: Core Setup & Validation (Sprint 1-2)**

###### **Tasks**

1. **Project Initialization** ✅

   - Set up Rust project with Actix/Axum. ✅
   - Configure CI/CD (GitHub Actions). ✅
   - **DoD**: Project builds successfully, CI pipeline passes. ✅

2. **Basic Email Validation** ✅

   - Implement syntax validation (regex). ✅
   - Add DNS/MX record verification. ✅
   - **DoD**: Unit tests cover 90% of cases, returns structured validation results. ✅

3. **MongoDB Integration & Disposable emails validation** ✅

   - Design database schema for disposable email domains. ✅
   - Implement disposable email addreses validation. ✅
   - **DoD**: DB migrations applied, test queries succeed. ✅

4. **REST API (Basic Endpoints)** ✅
   - Implement `POST /validate` for single email validation. ✅
   - Add error handling and OpenAPI docs. ✅
   - **DoD**: Endpoint tested via Postman, Swagger UI works. ✅

---

##### **Phase 2: GraphQL & Advanced Features (Sprint 3-4)**

###### **Tasks**

5. **GraphQL Integration** ✅

   - Set up GraphQL server (Async-GraphQL). ✅
   - Add `validateEmail` query and `validateEmailsBulk` mutation. ✅
   - **DoD**: GraphQL playground accessible, queries return correct responses. ✅

6. **Redis Caching Layer** ✅

   - Cache DNS/MX results to reduce latency. ✅
   - Implement TTL for cached entries. ✅
   - **DoD**: Cached responses are 50% faster than uncached ones. ✅

7. **Disposable & Role-Based Email Detection**

   - Integrate blocklists for disposable emails. ✅
   - Detect role-based addresses (e.g., `admin@`, `support@`). ✅
   - **DoD**: Blocklists loaded at startup, role detection accuracy >95%. ✅

8. **Bulk Processing**
   - Add async bulk validation REST endpoint (`POST /bulk/validate`). ✅
   - Implement job queue (Redis or MongoDB).
   - **DoD**: Processes 10K emails in <5 mins, returns job status.

---

##### **Phase 3: Performance & Security (Sprint 5-6)**

###### **Tasks**

9. **Authentication & Authorization**

   - Add JWT/API key authentication.
   - Restrict sensitive endpoints.
   - **DoD**: Unauthorized requests blocked, keys validated via DB.

10. **Rate Limiting**

    - Implement Redis-based rate limiting (per API key).
    - **DoD**: Rejects requests beyond 10 reqs/sec, logs violations.

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

---

![selfsend-github-bio](https://github.com/user-attachments/assets/66e57877-06d3-4156-b5d6-cd4a28f30c71)
